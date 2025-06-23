#![doc = include_str!("../README.md")]

use crate::filter::AsyncFilter;
use crate::iter::AsyncIterator;
use crate::map::AsyncMap;

pub mod filter;
pub mod iter;
pub mod map;
pub mod option;

pub trait AsyncIterTools: AsyncIterator {
  /// Calls an async closure on each element of an iterator
  ///
  /// This is equivalent to using a for loop on the iterator, although break and continue are not possible from a closure.
  ///
  /// ---
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```rust,ignore
  /// use async_iter_ext::AsyncIterTools;
  /// use futures::executor::block_on;
  /// use async_std::task::sleep;
  ///
  /// fn main() {
  ///     block_on(async {
  ///         let items = [1, 2, 3, 4];
  ///         items.iter().for_each_async(|item| async move {
  ///             // Simulate some async work
  ///             sleep(Duration::from_millis(100)).await;
  ///             println!("Processing {}", item);
  ///         }).await;
  ///     });
  /// }
  /// ```
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

  /// Applies an async closure to each item of the iterator, returning a new iterator
  /// of the results of each async computation.
  ///
  /// This is similar to the standard `Iterator::map` method, but works with asynchronous
  /// closures that return `Future`s.
  ///
  /// ---
  ///
  /// # Examples
  ///
  /// ```rust,ignore
  /// use async_iter_ext::AsyncIterTools;
  /// use futures::executor::block_on;
  ///
  /// fn main() {
  ///     block_on(async {
  ///         let items = [1, 2, 3, 4];
  ///         let mut mapped = items.iter().map_async(|item| async move {
  ///             // Simulate async transformation
  ///             item * 2
  ///         });
  ///
  ///         while let Some(result) = mapped.next_async().await {
  ///             println!("Result: {}", result);
  ///         }
  ///     });
  /// }
  /// ```
  fn map_async<B, F, Fut>(self, f: F) -> AsyncMap<Self, F>
  where
    Self: Sized,
    F: Fn(Self::Item) -> Fut,
    Fut: Future<Output = B>,
  {
    AsyncMap { iter: self, f }
  }

  /// Filters the items of an iterator using an asynchronous predicate.
  ///
  /// This works like the standard `Iterator::filter`, but allows the predicate
  /// to be asynchronous by returning a `Future<Output = bool>`.
  ///
  /// ---
  ///
  /// > ⚠️ Warning: The item type must implement `Clone` because filtering requires that the item is cloned when the predicate is run.
  ///
  /// ---
  ///
  /// # Examples
  ///
  /// ```rust,ignore
  /// use async_iter_ext::AsyncIterTools;
  /// use futures::executor::block_on;
  ///
  /// fn main() {
  ///     block_on(async {
  ///         let items = [1, 2, 3, 4, 5, 6];
  ///         let mut filtered = items.iter().filter_async(|item| async move {
  ///             // Keep even numbers only
  ///             item % 2 == 0
  ///         });
  ///
  ///         while let Some(item) = filtered.next_async().await {
  ///             println!("Filtered: {}", item);
  ///         }
  ///     });
  /// }
  /// ```
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
