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

#[derive(Clone)]
#[must_use = "async iterator combinators are lazy and do nothing unless consumed"]
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

impl<I, F> Debug for AsyncFilter<I, F>
where
    I: AsyncIterator + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncFilter")
            .field("iter", &self.iter)
            .finish()
    }
}
