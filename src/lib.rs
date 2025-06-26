#![doc = include_str!("../README.md")]

use combinator::{filter::AsyncFilter, map::AsyncMap};

pub mod combinator;
pub mod iter;
mod option;
mod result;

pub use iter::AsyncIterator;
pub use option::AsyncOptionTools;
pub use result::AsyncResultTools;

use crate::iter::process_result::ProcessResults;

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
    /// use std::time::Duration;
    ///
    /// use async_iter_ext::AsyncIterTools;
    /// use async_std::task;
    ///
    /// task::block_on(async {
    ///   let items = [1, 2, 3, 4];
    ///   items
    ///     .iter()
    ///     .for_each_async(|item| {
    ///       async move {
    ///         // Simulate some async work
    ///         task::sleep(Duration::from_millis(100)).await;
    ///         println!("Processing {}", item);
    ///       }
    ///     })
    ///     .await;
    /// });
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
    /// use std::time::Duration;
    ///
    /// use async_iter_ext::{AsyncIterTools, AsyncIterator};
    /// use async_std::task;
    ///
    /// let multiplied_by_two = task::block_on(async {
    ///   let items = [1, 2, 3, 4];
    ///   items
    ///     .iter()
    ///     .map_async(|item| {
    ///       async move {
    ///         // Simulate async transformation
    ///         task::sleep(Duration::from_millis(100)).await;
    ///         item * 2
    ///       }
    ///     })
    ///     .async_collect::<Vec<_>>()
    ///     .await
    /// });
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
    /// task::block_on(async {
    ///   let items = [1, 2, 3, 4, 5, 6];
    ///   let filtered = items
    ///     .iter()
    ///     .filter_async(|item| {
    ///       async move {
    ///         // Keep even numbers only
    ///         item % 2 == 0
    ///       }
    ///     })
    ///     .async_collect::<Vec<_>>();
    /// });
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

    /// Consumes the async iterator and returns a `ProcessResults` future that collects
    /// successes and errors based on a specified strategy.
    ///
    /// This is a convenience method for turning an `AsyncIterator<Item = Result<T, E>>`
    /// into a `ProcessResults`, which is a future that resolves to a
    /// `ProcessResultsContainer<T, E>`.
    ///
    /// By default, the `ProcessResultsStrategy::Partition` strategy is used,
    /// which means all items will be processed and separated into successes and errors.
    /// You can change the strategy using `.with_process_strategy(...)`.
    ///
    /// # Type Parameters
    /// - `T`: The success type inside the `Result`.
    /// - `E`: The error type inside the `Result`.
    ///
    /// # Returns
    /// A `ProcessResults<Self, T, E>` future that resolves to `ProcessResultsContainer<T, E>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use async_iter_ext::{AsyncIterTools, AsyncIterator, iter::process_result::ProcessResultsStrategy};
    /// use async_std::task;
    ///
    /// task::block_on(async {
    ///   // Partition strategy: collect all successes and errors
    ///   let results = vec![Ok(1), Err("err1"), Ok(2), Err("err2")]
    ///     .into_iter()
    ///     .process_results::<i32, &str>()
    ///     .await;
    ///   assert_eq!(results.successes(), &vec![1, 2]);
    ///   assert_eq!(results.errors(), &vec!["err1", "err2"]);
    ///
    ///   // Convert to result and unwrap error. OBS: Only gets the first error
    ///   let first_result_err = results.into_result().unwrap_err();
    ///   assert_eq!(first_result_err, "err1");
    ///
    ///   // Partition strategy: collect all successes
    ///   let results = vec![Ok(1), Ok(2)].into_iter().process_results::<i32, &str>().await;
    ///   assert_eq!(results.successes(), &vec![1, 2]);
    ///
    ///   // Convert to result and unwrap
    ///   let successes = results.into_result().unwrap();
    ///   assert_eq!(successes, vec![1, 2]);
    ///
    ///   // BreakOnError strategy: stop at the first error
    ///   let results = vec![Ok(1), Err("early"), Ok(2)]
    ///     .into_iter()
    ///     .process_results::<i32, &str>()
    ///     .with_process_strategy(ProcessResultsStrategy::BreakOnError)
    ///     .await;
    ///   assert_eq!(results.successes(), &vec![]); // did not continue after first error
    ///   assert_eq!(results.errors(), &vec!["early"]);
    /// });
    /// ```
    fn process_results<T, E>(self) -> ProcessResults<Self, T, E>
    where
        Self: Sized + AsyncIterator<Item = Result<T, E>>,
    {
        ProcessResults::new(self)
    }
}

impl<T> AsyncIterTools for T where T: AsyncIterator + ?Sized {}
