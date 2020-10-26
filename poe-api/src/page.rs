use futures::task::{Context, Poll};
use futures::Stream;
use std::future::Future;
use std::iter::Iterator;
use std::pin::Pin;

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

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

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
