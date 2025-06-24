use std::vec::IntoIter;
use sync_iter::SyncIter;

pub mod sync_iter;

pub trait AsyncIterator {
  type Item;

  fn next_async(&mut self) -> impl Future<Output = Option<Self::Item>>;

  fn sync_iter(&mut self) -> impl Future<Output = SyncIter<IntoIter<Self::Item>>>
  where
    Self: Sized,
  {
    async move { SyncIter::new(collect_into_vec(self).await.into_iter()) }
  }

  fn async_size_hint(&self) -> (usize, Option<usize>) {
    (0, None)
  }

  fn async_collect<B>(mut self) -> impl Future<Output = B>
  where
    Self: Sized,
    B: FromIterator<Self::Item>,
  {
    async move {
      let items = collect_into_vec(&mut self).await;
      B::from_iter(items)
    }
  }
}

async fn collect_into_vec<I>(iter: &mut I) -> Vec<I::Item>
where
  I: AsyncIterator,
{
  let mut items = vec![];

  match iter.async_size_hint() {
    (_, Some(upper_limit)) => {
      for _ in 0..upper_limit {
        if let Some(item) = iter.next_async().await {
          items.push(item);
        }
      }
    },
    _ => {
      while let Some(item) = iter.next_async().await {
        items.push(item);
      }
    },
  }

  items
}

impl<T> AsyncIterator for T
where
  T: Iterator + ?Sized,
{
  type Item = T::Item;

  async fn next_async(&mut self) -> Option<Self::Item> {
    self.next()
  }

  fn async_size_hint(&self) -> (usize, Option<usize>) {
    self.size_hint()
  }
}
