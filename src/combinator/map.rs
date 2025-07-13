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

/// An asynchronous iterator adapter that maps each item to a new value using an async function.
///
/// `AsyncMap` is similar to the standard `.map()` method on iterators, but it supports asynchronous closures
/// by allowing the mapping function to return a `Future`. This makes it useful for scenarios where each item
/// in an iterator needs to be processed using asynchronous logic.
///
/// This struct is created by the `.map_async()` method on `AsyncIterTools`.
///
/// # Type Parameters
/// - `I`: The underlying async iterator.
/// - `F`: The asynchronous mapping function, which produces a future for each item.
#[must_use = "async iterator combinators are lazy and do nothing unless consumed"]
pub struct AsyncMap<I, F> {
    pub(crate) iter: I,
    pub(crate) f: F,
}

/// Enables conversion of an `AsyncMap` into a synchronous `SyncIter` by polling the async iterator
/// and collecting items into a vector. Implements the `PollSyncIter` trait, allowing integration with
/// the `Future` implementation below.
impl<B, I, F, Fut> PollSyncIter for AsyncMap<I, F>
where
    I: AsyncIterator + Unpin,
    F: FnMut(I::Item) -> Fut + Unpin + Send,
    Fut: Future<Output = B> + Send,
{
}

/// Allows an `AsyncMap` to be `.await`ed directly, returning a synchronous iterator (`SyncIter`) over
/// the collected results of the async mapping operation. This makes it possible to use `.await` on
/// `AsyncMap` to collect all results at once in blocking contexts like `block_on`.
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

/// Implements the `AsyncIterator` trait for `AsyncMap`, yielding the result of applying
/// the async mapping function `f` to each item from the underlying iterator `iter`.
///
/// The `next_async()` method awaits the next item and then applies the mapping function,
/// awaiting its result before yielding it downstream.
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

/// Provides a `Debug` implementation for `AsyncMap` that includes debug output for the underlying iterator.
/// The mapping function `f` is not shown due to lack of generic support for `Debug` on closures.
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