use std::{
  fmt::{Debug, Formatter},
  ops::Deref,
  pin::{Pin, pin},
  task::{Context, Poll},
  vec::IntoIter,
};

use crate::iter::AsyncIterator;

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

impl<I> Debug for SyncIter<I>
where
  I: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("SyncIter").field("iter", &self.iter).finish()
  }
}
