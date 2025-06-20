use crate::iter::{AsyncIterator, PollSyncIter, SyncIter};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec::IntoIter;

#[derive(Clone)]
pub struct AsyncFilter<I, F> {
  pub(crate) iter: I,
  pub(crate) f: F,
}

impl<I, F, Fut> AsyncIterator for AsyncFilter<I, F>
where
  I: AsyncIterator,
  F: FnMut(I::Item) -> Fut,
  Fut: Future<Output = bool>,
  I::Item: Clone,
{
  type Item = I::Item;

  async fn next_async(&mut self) -> Option<Self::Item> {
    if let Some(next) = self.iter.next_async().await {
      (self.f)(next.clone()).await.then_some(next)
    } else {
      None
    }
  }

  fn async_size_hint(&self) -> (usize, Option<usize>) {
    self.iter.async_size_hint()
  }
}

impl<I, F, Fut> PollSyncIter for AsyncFilter<I, F>
where
  I: AsyncIterator,
  F: FnMut(I::Item) -> Fut,
  Fut: Future<Output = bool>,
  I::Item: Clone,
{
}

impl<I, F, Fut> Future for AsyncFilter<I, F>
where
  I: AsyncIterator + Unpin,
  F: FnMut(I::Item) -> Fut + Unpin + Send,
  Fut: Future<Output = bool> + Send,
  I::Item: Clone,
{
  type Output = SyncIter<IntoIter<I::Item>>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    Self::poll_sync_iter(self, cx)
  }
}

#[cfg(test)]
mod tests {
  use crate::AsyncIterTools;
  use crate::iter::AsyncIterator;
  use async_std::task::sleep;
  use rstest::rstest;
  use std::time::Duration;

  #[rstest]
  async fn test_simple_filter() {
    let items = vec![1, 2, 3];

    let filtered_items = items
      .into_iter()
      .filter_async(|i| async move {
        sleep(Duration::from_millis(100)).await;
        i == 2
      })
      .async_collect::<Vec<_>>()
      .await;

    assert_eq!(filtered_items, vec![2]);
  }

  #[rstest]
  async fn test_multiple_filters() {
    let items = vec![1, 2, 3];

    let filtered_items = items
      .into_iter()
      .filter_async(|i| async move {
        sleep(Duration::from_millis(100)).await;
        i > 1
      })
      .filter_async(|i| async move {
        sleep(Duration::from_millis(100)).await;
        i < 3
      })
      .async_collect::<Vec<_>>()
      .await;

    assert_eq!(filtered_items, vec![2]);
  }

  #[rstest]
  #[timeout(Duration::from_millis(400))]
  async fn test_async_filter_then_sync_filter() {
    let items = vec![1, 2, 3];

    let filtered_items = items
      .into_iter()
      .filter_async(|i| async move {
        sleep(Duration::from_millis(100)).await;
        i > 1
      })
      .await
      .filter(|i| *i < 3)
      .collect::<Vec<_>>();

    assert_eq!(filtered_items, vec![2]);
  }
}
