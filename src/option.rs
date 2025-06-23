pub trait AsyncOptionTools<T> {
  #[allow(clippy::wrong_self_convention)]
  fn is_some_and_async<F, Fut>(self, f: F) -> impl Future<Output = bool>
  where
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = bool>;

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
    if let Some(x) = self { f(x).await } else { false }
  }

  async fn map_async<B, F, Fut>(self, f: F) -> Option<B>
  where
    F: FnOnce(T) -> Fut,
    Fut: Future<Output = B>,
  {
    if let Some(x) = self { Some(f(x).await) } else { None }
  }
}
