/// An extension trait for `Result<T, E>` providing asynchronous combinators.
pub trait AsyncResultTools<T, E> {
  /// Asynchronously evaluates whether the result is `Ok` and satisfies a given predicate.
  ///
  /// If the result is `Ok`, the provided asynchronous predicate `f` is applied to the inner value.
  /// Returns `true` if the predicate resolves to `true`, otherwise returns `false`.
  /// If the result is `Err`, returns `false` without calling the predicate.
  ///
  /// # Example
  ///
  /// ```rust
  /// use async_iter_ext::AsyncResultTools;
  /// use async_std::task::block_on;
  ///
  /// async fn check_even(n: u32) -> bool {
  ///     n % 2 == 0
  /// }
  ///
  /// let res: Result<u32, ()> = Ok(4);
  /// assert!(block_on(async {res.is_ok_and_async(check_even).await}));
  /// ```
  #[allow(clippy::wrong_self_convention)]
  fn is_ok_and_async<F, Fut>(self, f: F) -> impl Future<Output = bool>
  where
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = bool>;

  /// Applies an asynchronous transformation to the `Ok` value, if present.
  ///
  /// If the result is `Ok`, the provided asynchronous function `f` is applied to the inner value,
  /// and the result is wrapped in `Ok`. If the result is `Err`, it is returned unchanged.
  ///
  /// # Example
  ///
  /// ```rust
  /// use async_iter_ext::AsyncResultTools;
  /// use async_std::task::block_on;
  ///
  /// async fn double(x: u32) -> u32 {
  ///     x * 2
  /// }
  ///
  /// let res: Result<u32, &str> = Ok(3);
  /// let doubled = block_on(async {res.map_async(double).await});
  /// assert_eq!(doubled, Ok(6));
  /// ```
  fn map_async<B, F, Fut>(self, f: F) -> impl Future<Output = Result<B, E>>
  where
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = B>;
}

impl<T, E> AsyncResultTools<T, E> for Result<T, E> {
  async fn is_ok_and_async<F, Fut>(self, f: F) -> bool
  where
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = bool>,
  {
    if let Ok(x) = self { f(x).await } else { false }
  }

  async fn map_async<B, F, Fut>(self, f: F) -> Result<B, E>
  where
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = B>,
  {
    match self {
      Ok(x) => Ok(f(x).await),
      Err(e) => Err(e),
    }
  }
}
