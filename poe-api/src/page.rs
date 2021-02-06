use futures::stream::FuturesOrdered;
use futures::task::{Context, Poll};
use futures::{ready, Stream, StreamExt};
use std::future::Future;
use std::iter::Iterator;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct PageRequest {
    pub limit: usize,
    pub offset: usize,
}

pub struct PagedStream<F, Fut, Iter, T, E>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Result<Iter, E>>,
    Iter: Iterator<Item = T>,
{
    worker_queue: FuturesOrdered<Fut>,
    current_workload: usize,
    current_iter: Option<Iter>,
    errored: bool,
    active_futures: usize,
    parallelism: usize,
    limit: usize,
    max: Option<usize>,
    pager: F,
}

impl<F, Fut, Iter, T, E> PagedStream<F, Fut, Iter, T, E>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Result<Iter, E>>,
    Iter: Iterator<Item = T>,
{
    pub fn new(parallelism: usize, limit: usize, max: Option<usize>, pager: F) -> Self {
        Self {
            worker_queue: FuturesOrdered::new(),
            current_workload: 0,
            current_iter: None,
            errored: false,
            active_futures: 0,
            parallelism,
            limit,
            max,
            pager,
        }
    }
}

impl<F, Fut, Iter, T, E> Stream for PagedStream<F, Fut, Iter, T, E>
where
    F: Fn(PageRequest) -> Fut,
    Fut: Future<Output = Result<Iter, E>>,
    Iter: Iterator<Item = T>,
{
    type Item = Result<T, E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.errored {
            return Poll::Ready(None);
        }

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
            item = match ready!(this.worker_queue.poll_next_unpin(cx)) {
                Some(Ok(elements)) => {
                    this.active_futures -= 1;
                    this.current_iter = Some(elements);
                    this.current_iter.as_mut().unwrap().next()
                }
                Some(Err(err)) => {
                    this.errored = true;
                    return Poll::Ready(Some(Err(err)));
                }
                None => None,
            };
        }

        Poll::Ready(item.map(Ok))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::TryStreamExt;

    #[tokio::test]
    async fn test() {
        let mut ps = PagedStream::new(5, 5, Some(100), |pr| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            Ok::<_, ()>(pr.offset..pr.offset + pr.limit)
        });

        let mut counter = 0;
        while let Some(x) = ps.try_next().await.unwrap() {
            assert_eq!(x, counter);
            counter += 1;
        }
        assert_eq!(counter, 100);
    }

    #[tokio::test]
    async fn test_error() {
        let mut ps = PagedStream::new(5, 1, Some(10), |pr| async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            if pr.offset > 3 {
                Err("error")
            } else {
                Ok(pr.offset..pr.offset + pr.limit)
            }
        });

        assert_eq!(0, ps.try_next().await.unwrap().unwrap());
        assert_eq!(1, ps.try_next().await.unwrap().unwrap());
        assert_eq!(2, ps.try_next().await.unwrap().unwrap());
        assert_eq!(3, ps.try_next().await.unwrap().unwrap());
        assert_eq!(Err("error"), ps.try_next().await);
        assert_eq!(None, ps.try_next().await.unwrap());
    }
}
