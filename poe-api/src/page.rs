use futures::channel::mpsc;
use futures::sink::SinkExt;
use futures::stream::FuturesOrdered;
use futures::task::{Context, Poll, Spawn, SpawnExt};
use futures::{Stream, StreamExt};
use std::future::Future;
use std::iter::Iterator;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct PageRequest {
    pub limit: usize,
    pub offset: usize,
}

#[derive(Debug)]
pub struct PagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Option<Iter>>,
    Iter: Iterator<Item = T>,
{
    pager: F,
    page_size: usize,
    max: Option<usize>,
    current_offset: usize,
    current_future: Option<Pin<Box<Fut>>>,
    current_iter: Option<Iter>,
}

impl<F, Fut, Iter, T> PagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Option<Iter>>,
    Iter: Iterator<Item = T>,
{
    pub fn new(page_size: usize, max: Option<usize>, pager: F) -> Self {
        PagedStream {
            pager,
            page_size,
            max,
            current_offset: 0,
            current_future: None,
            current_iter: None,
        }
    }
}

impl<F, Fut, Iter, T> Stream for PagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Option<Iter>>,
    Iter: Iterator<Item = T>,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = unsafe { self.get_unchecked_mut() };

        let mut item = this.current_iter.as_mut().and_then(|iter| iter.next());

        // no iterator or iterator yielded no item
        if item.is_none() {
            let mut fut = if let Some(fut) = this.current_future.take() {
                fut
            } else {
                let current_offset = this.current_offset;
                if this.max.map_or(false, |max| current_offset >= max) {
                    return Poll::Ready(None);
                }

                this.current_offset += this.page_size;
                Box::pin((this.pager)(PageRequest {
                    limit: this.page_size,
                    offset: current_offset,
                }))
            };

            item = match fut.as_mut().poll(cx) {
                Poll::Ready(Some(elements)) => {
                    this.current_iter = Some(elements);
                    this.current_iter.as_mut().unwrap().next()
                }
                // future returned none -> no more elements, terminate
                Poll::Ready(None) => None,
                Poll::Pending => {
                    this.current_future = Some(fut);
                    return Poll::Pending;
                }
            };
        }

        Poll::Ready(item)
    }
}

pub struct ParallelPagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Option<Iter>>,
    Iter: Iterator<Item = T>,
{
    executor: Box<dyn Spawn>,
    pager: Arc<F>,
    receiver: Option<mpsc::Receiver<T>>,
    workers: usize,
    limit: usize,
    max: Option<usize>,
}

impl<F, Fut, Iter, T> ParallelPagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<Iter>> + Send,
    Iter: Iterator<Item = T> + Send,
{
    pub fn new(workers: usize, limit: usize, max: Option<usize>, pager: F) -> Self {
        Self {
            executor: Box::new(TokioSpawner {}),
            pager: Arc::new(pager),
            receiver: None,
            workers,
            limit,
            max,
        }
    }
}

impl<F, Fut, Iter, T> Stream for ParallelPagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Option<Iter>> + Send,
    Iter: Iterator<Item = T> + Send,
    T: Send + 'static,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.receiver.is_none() {
            let (tx_data, rx_data) = mpsc::channel::<T>(this.workers * this.limit);
            let (tx_sync, _) = broadcast::channel::<usize>(this.workers);

            for id in 0..this.workers {
                let workers = this.workers;
                let limit = this.limit;
                let max = this.max;
                let mut rx_sync = tx_sync.subscribe();
                let tx_sync = tx_sync.clone();
                let mut tx_data = tx_data.clone();
                let pager = Arc::clone(&this.pager);
                // let pager = &this.pager;

                let fut = async move {
                    for i in 0.. {
                        let workload = id + workers * i;
                        let start = workload * limit;

                        if start >= max.unwrap_or(usize::MAX) {
                            break;
                        }

                        let data = match pager(PageRequest {
                            offset: start,
                            limit,
                        })
                        .await
                        {
                            Some(data) => data,
                            None => break,
                        };

                        while rx_sync.recv().await.unwrap() != workload {}

                        for item in data {
                            if let Err(error) = tx_data.send(item).await {
                                if error.is_disconnected() {
                                    break;
                                }
                            }
                        }

                        tx_sync.send(workload + 1).unwrap();
                    }
                };

                this.executor.spawn(fut).unwrap();
            }
            // bootstrap and start with the first workload
            tx_sync.send(0).unwrap();

            this.receiver = Some(rx_data);
        };

        this.receiver.as_mut().unwrap().poll_next_unpin(cx)
    }
}

struct TokioSpawner {}

impl futures::task::Spawn for TokioSpawner {
    fn spawn_obj(
        &self,
        future: futures::task::FutureObj<'static, ()>,
    ) -> Result<(), futures::task::SpawnError> {
        tokio::spawn(future);
        Ok(())
    }
}

pub struct GoodPagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Option<Iter>>,
    Iter: Iterator<Item = T>,
{
    worker_queue: FuturesOrdered<Fut>,
    current_workload: usize,
    current_iter: Option<Iter>,
    active_futures: usize,
    parallelism: usize,
    limit: usize,
    max: Option<usize>,
    pager: F,
}

impl<F, Fut, Iter, T> GoodPagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Option<Iter>>,
    Iter: Iterator<Item = T>,
{
    pub fn new(parallelism: usize, limit: usize, max: Option<usize>, pager: F) -> Self {
        Self {
            worker_queue: FuturesOrdered::new(),
            current_workload: 0,
            current_iter: None,
            active_futures: 0,
            parallelism,
            limit,
            max,
            pager,
        }
    }
}

impl<F, Fut, Iter, T> Stream for GoodPagedStream<F, Fut, Iter, T>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Option<Iter>>,
    Iter: Iterator<Item = T>,
{
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = unsafe { self.get_unchecked_mut() };

        while this.active_futures < this.parallelism {
            let offset = this.current_workload * this.limit;

            if offset >= this.max.unwrap_or(usize::MAX) {
                break;
            }

            this.worker_queue.push((this.pager)(PageRequest {
                limit: this.limit,
                offset,
            }));
            this.current_workload += 1;
            this.active_futures += 1;
        }

        let mut item = this.current_iter.as_mut().and_then(|iter| iter.next());

        if item.is_none() {
            item = match this.worker_queue.poll_next_unpin(cx) {
                Poll::Ready(Some(elements)) => {
                    this.active_futures -= 1;
                    this.current_iter = elements;
                    this.current_iter.as_mut().unwrap().next()
                }
                Poll::Ready(None) => None,
                Poll::Pending => return Poll::Pending,
            };
        }

        Poll::Ready(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_good_parallel() {
        let mut ps = GoodPagedStream::new(5, 10, Some(100), |pr| async move {
            tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;
            Some(pr.offset..pr.offset + pr.limit)
        });

        let mut counter = 0;
        while let Some(x) = ps.next().await {
            assert_eq!(x, counter);
            counter += 1;
        }
        assert_eq!(counter, 100);
    }

    #[tokio::test]
    async fn test_parallel() {
        let mut ps = ParallelPagedStream::new(5, 10, Some(100), move |pr| async move {
            tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;
            Some(pr.offset..pr.offset + pr.limit)
        });

        let mut counter = 0;
        while let Some(x) = ps.next().await {
            assert_eq!(x, counter);
            counter += 1;
        }
        assert_eq!(counter, 100);
    }

    #[tokio::test]
    async fn test() {
        let mut ps = PagedStream::new(5, Some(20), |pr| async move {
            assert_eq!(pr.offset % 5, 0);
            assert_eq!(pr.limit, 5);
            tokio::time::delay_for(tokio::time::Duration::from_millis(1)).await;
            Some(pr.offset..pr.offset + pr.limit)
        });

        let mut counter: usize = 0;
        while let Some(x) = ps.next().await {
            assert_eq!(x, counter);
            counter += 1;
        }
        assert_eq!(counter, 20);
    }
}
