/// Asynchronous extension methods for `Option<T>`.
///
/// This trait allows using asynchronous functions with `Option` types,
pub trait AsyncOptionTools<T> {
    /// Asynchronously checks if the option is `Some` and satisfies a predicate.
    ///
    /// Returns `false` if the value is `None`. If `Some`, the provided async function is run.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::task;
    /// use async_iter_ext::AsyncOptionTools;
    ///
    /// #[test]
    /// fn test_is_some_and_async() {
    ///     task::block_on(async {
    ///         async fn is_positive(x: i32) -> bool {
    ///             x > 0
    ///         }
    ///
    ///         let some = Some(10);
    ///         let none: Option<i32> = None;
    ///
    ///         assert_eq!(some.is_some_and_async(is_positive).await, true);
    ///         assert_eq!(none.is_some_and_async(is_positive).await, false);
    ///     });
    /// }
    /// ```
    fn is_some_and_async<F, Fut>(self, f: F) -> impl Future<Output = bool>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = bool>;

    /// Asynchronously checks if the option is `None` or satisfies a predicate.
    ///
    /// - Returns `true` if the value is `None`.
    /// - If `Some`, runs the async predicate and returns its result.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::task;
    /// use async_iter_ext::AsyncOptionTools;
    ///
    /// #[test]
    /// fn test_is_none_or_async() {
    ///     task::block_on(async {
    ///         async fn is_zero(x: u8) -> bool {
    ///             x == 0
    ///         }
    ///
    ///         let some = Some(0);
    ///         let none: Option<u8> = None;
    ///
    ///         assert_eq!(some.is_none_or_async(is_zero).await, true);
    ///         assert_eq!(some.is_none_or_async(|x| async move { x > 1 }).await, false);
    ///         assert_eq!(none.is_none_or_async(is_zero).await, true);
    ///     });
    /// }
    /// ```
    fn is_none_or_async<F, Fut>(self, f: F) -> impl Future<Output = bool>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = bool>;

    /// Asynchronously maps an `Option<T>` to an `Option<B>` using an async function.
    ///
    /// - If `Some`, the function is awaited and wrapped in `Some`.
    /// - If `None`, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::task;
    /// use async_iter_ext::AsyncOptionTools;
    ///
    /// #[test]
    /// fn test_map_async() {
    ///     task::block_on(async {
    ///         async fn to_string_async(n: i32) -> String {
    ///             format!("Number: {}", n)
    ///         }
    ///
    ///         let some = Some(42);
    ///         let result = some.map_async(to_string_async).await;
    ///         assert_eq!(result, Some("Number: 42".to_string()));
    ///
    ///         let none: Option<i32> = None;
    ///         let result = none.map_async(to_string_async).await;
    ///         assert_eq!(result, None);
    ///     });
    /// }
    /// ```
    fn map_async<B, F, Fut>(self, f: F) -> impl Future<Output = Option<B>>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = B>;
}

impl<T> AsyncOptionTools<T> for Option<T> {
    async fn is_some_and_async<F, Fut>(self, f: F) -> bool
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = bool>,
    {
        if let Some(x) = self {
            f(x).await
        } else {
            false
        }
    }

    async fn is_none_or_async<F, Fut>(self, f: F) -> bool
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = bool>,
    {
        if let Some(x) = self { f(x).await } else { true }
    }

    async fn map_async<B, F, Fut>(self, f: F) -> Option<B>
    where
        F: FnOnce(T) -> Fut,
        Fut: Future<Output = B>,
    {
        if let Some(x) = self {
            Some(f(x).await)
        } else {
            None
        }
    }
}
