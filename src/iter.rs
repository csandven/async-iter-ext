use std::vec::IntoIter;

use sync_iter::SyncIter;

pub mod process_result;
pub mod sync_iter;

/// Trait for asynchronous iteration.
///
/// This trait is similar to the standard `Iterator` trait, but designed to work in asynchronous
/// contexts where `next()` returns a `Future`. It provides core methods for driving async iteration,
/// collecting results, and converting to a synchronous iterator.
///
/// Implementors must define `next_async`, which yields the next item wrapped in a `Future`.
pub trait AsyncIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Asynchronously returns the next item in the iterator.
    ///
    /// Returns `None` when iteration is finished.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use async_iter_ext::AsyncIterator;
    ///
    /// async fn iterate<I: AsyncIterator>(mut iter: I) {
    ///     while let Some(item) = iter.next_async().await {
    ///         // process item
    ///     }
    /// }
    /// ```
    fn next_async(&mut self) -> impl Future<Output = Option<Self::Item>>;

    /// Converts this async iterator into a synchronous [`SyncIter`] using `async` collection.
    ///
    /// This method collects all remaining items into a `Vec` and returns a `SyncIter` over them.
    ///
    /// ---
    ///
    /// # Notes
    ///
    /// - This is useful when you want to move from an async context to sync processing.
    ///
    /// ---
    ///
    /// # Examples
    ///
    /// ```rust
    /// use async_iter_ext::AsyncIterator;
    ///
    /// async fn convert<I: AsyncIterator>(iter: &mut I) {
    ///     let sync = iter.sync_iter().await;
    ///     for item in sync {
    ///         // use item
    ///     }
    /// }
    /// ```
    fn sync_iter(&mut self) -> impl Future<Output = SyncIter<IntoIter<Self::Item>>>
    where
        Self: Sized,
    {
        async move { SyncIter::new(collect_into_vec(self).await.into_iter()) }
    }

    /// Provides a hint about the size of the remaining items.
    ///
    /// Returns a tuple where the first element is a lower bound and the second
    /// is an optional upper bound.
    ///
    /// This is used to optimize certain operations such as preallocating capacity.
    ///
    /// Default implementation returns `(0, None)`.
    fn async_size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }

    /// Collects all items of the async iterator into a container type.
    ///
    /// Works like the standard `Iterator::collect`, but in an async context.
    ///
    /// The target container must implement `FromIterator<Self::Item>`.
    ///
    /// ---
    ///
    /// # Examples
    ///
    /// ```rust
    /// use async_iter_ext::AsyncIterator;
    ///
    /// async fn collect_items<I: AsyncIterator>(iter: I) -> Vec<I::Item> {
    ///     iter.async_collect::<Vec<_>>().await
    /// }
    /// ```
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

/// Asynchronously collects all items from an [`AsyncIterator`] into a `Vec`.
///
/// Uses `async_size_hint` to optimize preallocation if the upper bound is known.
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
        }
        _ => {
            while let Some(item) = iter.next_async().await {
                items.push(item);
            }
        }
    }

    items
}

/// Blanket implementation of [`AsyncIterator`] for all synchronous [`Iterator`] types.
///
/// This enables any standard iterator to be used as an async iterator by immediately
/// returning the next item.
///
/// Note: Since this is a synchronous fallback, `next_async()` will not perform actual async work.
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
