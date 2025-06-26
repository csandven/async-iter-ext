use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
    pin::{Pin, pin},
    task::{Context, Poll},
};

use crate::AsyncIterator;

/// Defines the strategy to use when processing results from an asynchronous iterator.
#[derive(Default, Clone, Debug)]
pub enum ProcessResultsStrategy {
    /// Continue processing all results, separating successes and errors.
    #[default]
    Partition,

    /// Stop processing at the first error encountered.
    BreakOnError,
}

/// A container that holds both successful and erroneous results.
pub struct ProcessResultsContainer<T, E> {
    successes: Vec<T>,
    errors: Vec<E>,
}

impl<T, E> Deref for ProcessResultsContainer<T, E> {
    type Target = Vec<T>;

    /// Dereferences to the vector of successful results.
    fn deref(&self) -> &Self::Target {
        self.successes()
    }
}

impl<T, E> ProcessResultsContainer<T, E> {
    /// Converts the container into a `Result`, returning `Ok` with successes,
    /// or the first `Err` if there are any errors.
    pub fn into_result(self) -> Result<Vec<T>, E> {
        if !self.errors.is_empty() {
            Err(self.into_errors().remove(0))
        } else {
            Ok(self.successes)
        }
    }

    /// Consumes the container and returns the vector of successes.
    pub fn into_successes(self) -> Vec<T> {
        self.successes
    }

    /// Consumes the container and returns the vector of errors.
    pub fn into_errors(self) -> Vec<E> {
        self.errors
    }

    /// Returns a reference to the vector of successes.
    pub fn successes(&self) -> &Vec<T> {
        self.successes.as_ref()
    }

    /// Returns a reference to the vector of errors.
    pub fn errors(&self) -> &Vec<E> {
        self.errors.as_ref()
    }
}

impl<T, E> Debug for ProcessResultsContainer<T, E>
where
    T: Debug,
    E: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessResultsContainer")
            .field("successes", &self.successes)
            .field("errors", &self.errors)
            .finish()
    }
}

impl<T, E> Clone for ProcessResultsContainer<T, E>
where
    T: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        Self {
            successes: self.successes.clone(),
            errors: self.errors.clone(),
        }
    }
}

impl<T, E> From<(Vec<T>, Vec<E>)> for ProcessResultsContainer<T, E> {
    /// Creates a `ProcessResultsContainer` from a tuple of successes and errors.
    fn from((successes, errors): (Vec<T>, Vec<E>)) -> Self {
        Self { successes, errors }
    }
}

/// A future that processes results from an asynchronous iterator,
/// collecting successes and errors based on the specified strategy.
pub struct ProcessResults<I, T, E>
where
    I: AsyncIterator<Item = Result<T, E>>,
{
    iter: I,
    strategy: ProcessResultsStrategy,
}

impl<I, T, E> ProcessResults<I, T, E>
where
    I: AsyncIterator<Item = Result<T, E>>,
{
    /// Constructs a new `ProcessResults` future using the given async iterator.
    pub fn new(iter: I) -> ProcessResults<I, T, E> {
        Self {
            iter,
            strategy: ProcessResultsStrategy::default(),
        }
    }

    /// Sets the processing strategy to use for handling errors during iteration.
    pub fn with_process_strategy(mut self, strategy: ProcessResultsStrategy) -> Self {
        self.strategy = strategy;
        self
    }
}

impl<I, T, E> Future for ProcessResults<I, T, E>
where
    I: AsyncIterator<Item = Result<T, E>> + Unpin,
    T: Unpin,
    E: Unpin,
{
    type Output = ProcessResultsContainer<T, E>;

    /// Polls the future and returns a container of results.
    /// Depending on the strategy, it either:
    /// - `Partition`: Collects all successes and errors.
    /// - `BreakOnError`: Stops at the first error and returns it immediately.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let strategy = self.strategy.clone();
        let mut pinned_fut = pin!(self.get_mut().iter.sync_iter());

        loop {
            match pinned_fut.as_mut().poll(cx) {
                Poll::Pending => {}
                Poll::Ready(res) => {
                    let mut successes = vec![];
                    let mut errors = vec![];

                    for item in res {
                        match item {
                            Ok(item) => successes.push(item),
                            Err(error) => {
                                errors.push(error);
                                match strategy {
                                    ProcessResultsStrategy::Partition => {}
                                    ProcessResultsStrategy::BreakOnError => {
                                        return Poll::Ready((vec![], errors).into());
                                    }
                                }
                            }
                        }
                    }

                    return Poll::Ready((successes, errors).into());
                }
            }
        }
    }
}

impl<I, T, E> Debug for ProcessResults<I, T, E>
where
    I: AsyncIterator<Item = Result<T, E>> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProcessResults")
            .field("iter", &self.iter)
            .field("strategy", &self.strategy)
            .finish()
    }
}
