#![doc = include_str!("../README.md")]

use combinator::filter::AsyncFilter;
use combinator::map::AsyncMap;

pub mod combinator;
pub mod iter;
mod option;
mod result;

pub use option::AsyncOptionTools;
pub use result::AsyncResultTools;
pub use iter::AsyncIterator;

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
  /// ```rust
  /// use async_iter_ext::AsyncIterTools;
  /// use async_std::task;
  /// use std::time::Duration;
  ///
  /// fn main() {
  ///     task::block_on(async {
  ///         let items = [1, 2, 3, 4];
  ///         items.iter().for_each_async(|item| async move {
  ///             // Simulate some async work
  ///             task::sleep(Duration::from_millis(100)).await;
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
  /// ```rust
  /// use async_iter_ext::{AsyncIterTools, AsyncIterator};
  /// use async_std::task;
  /// use std::time::Duration;
  ///
  /// fn main() {
  ///   let multiplied_by_two = task::block_on(async {
  ///     let items = [1, 2, 3, 4];
  ///     items.iter().map_async(|item| async move {
  ///       // Simulate async transformation
  ///       task::sleep(Duration::from_millis(100)).await;
  ///       item * 2
  ///     }).async_collect::<Vec<_>>().await
  ///   });
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
  /// ```rust
  /// use async_iter_ext::{AsyncIterTools, AsyncIterator}; 
  /// use async_std::task;
  ///
  /// fn main() {
  ///   task::block_on(async {
  ///     let items = [1, 2, 3, 4, 5, 6];
  ///     let filtered = items.iter().filter_async(|item| async move {
  ///       // Keep even numbers only
  ///       item % 2 == 0
  ///     }).async_collect::<Vec<_>>();
  ///   });
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
