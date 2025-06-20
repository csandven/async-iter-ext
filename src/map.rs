use crate::iter::{AsyncIterator, PollSyncIter, SyncIter};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::vec::IntoIter;

pub struct AsyncMap<I, F> {
  pub(crate) iter: I,
  pub(crate) f: F,
}

impl<B, I, F, Fut> PollSyncIter for AsyncMap<I, F>
where
  I: AsyncIterator + Unpin,
  F: FnMut(I::Item) -> Fut + Unpin + Send,
  Fut: Future<Output = B> + Send,
{
}

impl<B, I, F, Fut> Future for AsyncMap<I, F>
where
  I: AsyncIterator + Unpin,
  F: FnMut(I::Item) -> Fut + Unpin + Send,
  Fut: Future<Output = B> + Send,
{
  type Output = SyncIter<IntoIter<B>>;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    Self::poll_sync_iter(self, cx)
  }
}

impl<B, I, F, Fut> AsyncIterator for AsyncMap<I, F>
where
  I: AsyncIterator,
  F: FnMut(I::Item) -> Fut + Send,
  Fut: Future<Output = B> + Send,
{
  type Item = B;

  async fn next_async(&mut self) -> Option<Self::Item> {
    if let Some(next) = self.iter.next_async().await {
      Some((self.f)(next).await)
    } else {
      None
    }
  }

  fn async_size_hint(&self) -> (usize, Option<usize>) {
    self.iter.async_size_hint()
  }
}

#[cfg(test)]
mod tests {
  use crate::AsyncIterTools;
  use crate::iter::AsyncIterator;
  use rstest::rstest;

  use async_std::task::sleep;
  use std::ops::Mul;
  use std::time::Duration;

  async fn multiply_item<T: Mul<i32, Output = i32>>(value: T) -> i32 {
    sleep(Duration::from_millis(100)).await;
    value * 2
  }

  #[rstest]
  async fn test_single_multiply() {
    let items = [1, 2, 3, 4];

    let mapped_items_vec = items.iter().map_async(multiply_item).async_collect::<Vec<_>>().await;

    assert_eq!(mapped_items_vec.len(), items.len());
    assert_eq!(mapped_items_vec, vec![2, 4, 6, 8]);
  }

  #[rstest]
  async fn test_multiple_multiply() {
    let items = [1, 2, 3, 4];

    let mapped_items_vec = items
      .iter()
      .map_async(multiply_item)
      .map_async(multiply_item)
      .map_async(multiply_item)
      .async_collect::<Vec<_>>()
      .await;

    assert_eq!(mapped_items_vec.len(), items.len());
    assert_eq!(mapped_items_vec, vec![8, 16, 24, 32]);
  }

  #[rstest]
  #[timeout(Duration::from_millis(400))]
  async fn test_async_map_then_sync_map() {
    let items = [1, 2, 3, 4];

    let async_and_then_sync = items
      .iter()
      .map_async(multiply_item)
      .await
      .map(|item| item * 2)
      .collect::<Vec<_>>();

    assert_eq!(async_and_then_sync.len(), items.len());
    assert_eq!(async_and_then_sync, vec![4, 8, 12, 16]);
  }
}
