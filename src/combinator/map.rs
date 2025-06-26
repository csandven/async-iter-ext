use std::{
    fmt::{Debug, Formatter},
    pin::Pin,
    task::{Context, Poll},
    vec::IntoIter,
};

use crate::iter::{
    AsyncIterator,
    sync_iter::{PollSyncIter, SyncIter},
};

#[must_use = "async iterator combinators are lazy and do nothing unless consumed"]
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

impl<I, F> Debug for AsyncMap<I, F>
where
    I: AsyncIterator + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncMap")
            .field("iter", &self.iter)
            .finish()
    }
}
