use std::ops::Deref;
use std::pin::{Pin, pin};
use std::task::{Context, Poll};
use std::vec::IntoIter;

pub trait PollSyncIter: AsyncIterator {
  #[inline]
  fn poll_sync_iter(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<SyncIter<IntoIter<Self::Item>>>
  where
    Self: Sized + Unpin,
  {
    // Pin the future so it can be polled.
    let mut pinned_fut = pin!(self.get_mut().sync_iter());

    loop {
      match pinned_fut.as_mut().poll(cx) {
        Poll::Pending => {},
        Poll::Ready(res) => return Poll::Ready(res),
      }
    }
  }
}

pub struct SyncIter<I> {
  iter: I,
}

impl<I> SyncIter<I> {
  pub(crate) fn new(iter: I) -> Self {
    Self { iter }
  }
}

impl<I> Deref for SyncIter<I> {
  type Target = I;

  fn deref(&self) -> &Self::Target {
    &self.iter
  }
}

impl<I> Iterator for SyncIter<I>
where
  I: Iterator,
{
  type Item = I::Item;

  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next()
  }
}

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

// pub type ArcedMap<T> = Map<T, fn(<T as Iterator>::Item) -> Arc<<T as Iterator>::Item>>;
//
// pub trait MapIntoArc<I: Iterator = Self> {
//   fn arc_iter(self) -> ArcedMap<I>
//   where
//     Self: Sized;
// }
//
// impl<T> MapIntoArc for T
// where
//   T: Iterator,
// {
//   fn arc_iter(self) -> ArcedMap<Self>
//   where
//     Self: Sized,
//   {
//     self.map(Arc::new)
//   }
// }
//
// impl<T> MapIntoArc<IntoIter<T>> for Vec<T> {
//   fn arc_iter(self) -> ArcedMap<IntoIter<T>>
//   where
//     Self: Sized,
//   {
//     self.into_iter().map(Arc::new)
//   }
// }
//
// #[cfg(test)]
// mod tests {
//   use crate::AsyncIterTools;
//   use crate::iter::MapIntoArc;
//   use rstest::rstest;
//   use std::sync::Arc;
//
//   #[test]
//   fn test_map_into_arc_iter() {
//     let items = vec![1, 2, 3];
//
//
//     items.iter().for_each(|item| {});
//
//     let arced = items.into_iter().arc_iter().collect::<Vec<_>>();
//
//     assert_eq!(arced, vec![Arc::new(1), Arc::new(2), Arc::new(3)]);
//   }
//
//   #[test]
//   fn test_map_into_arc_vec() {
//     let items = vec![1, 2, 3];
//     let arced = items.arc_iter().collect::<Vec<_>>();
//
//     assert_eq!(arced, vec![Arc::new(1), Arc::new(2), Arc::new(3)]);
//   }
//
//   #[rstest]
//   async fn test_map_into_arc_then_async_for_each() {
//     let items = vec![0, 1, 2];
//     items
//       .into_iter()
//       .arc_iter()
//       .enumerate()
//       .for_each_async(|(index, item)| async move {
//         assert_eq!(Arc::new(index as i32), item);
//       })
//       .await;
//   }
// }
