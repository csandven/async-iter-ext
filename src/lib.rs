use crate::filter::AsyncFilter;
use crate::iter::AsyncIterator;
use crate::map::AsyncMap;

mod filter;
mod iter;
mod map;

pub trait AsyncIterTools: AsyncIterator {
  fn for_each_async<F, Fut>(mut self, f: F) -> impl Future<Output = ()>
  where
    Self: Sized,
    F: Fn(Self::Item) -> Fut,
    Fut: Future<Output = ()>,
  {
    async move {
      while let Some(item) = self.next_async().await {
        f(item).await;
      }
    }
  }

  fn map_async<B, F, Fut>(self, f: F) -> AsyncMap<Self, F>
  where
    Self: Sized,
    F: Fn(Self::Item) -> Fut,
    Fut: Future<Output = B>,
  {
    AsyncMap { iter: self, f }
  }

  fn filter_async<F, Fut>(self, f: F) -> AsyncFilter<Self, F>
  where
    Self: Sized,
    F: Fn(Self::Item) -> Fut,
    Fut: Future<Output = bool>,
    Self::Item: Clone,
  {
    AsyncFilter { iter: self, f }
  }
}

impl<T> AsyncIterTools for T where T: AsyncIterator + ?Sized {}

#[cfg(test)]
mod tests {
  use crate::AsyncIterTools;

  #[rstest::rstest]
  async fn test_for_each_async_mut() {
    let items = [1, 2, 3, 4];

    let mut mut_items = items;
    mut_items
      .iter_mut()
      .for_each_async(|item| async move {
        *item += 3;
      })
      .await;

    assert_eq!(mut_items.len(), items.len());
    assert_eq!(mut_items, [4, 5, 6, 7]);
  }
}
