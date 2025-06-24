use async_iter_ext::AsyncOptionTools;
use async_std::task::sleep;
use rstest::rstest;
use std::time::Duration;

#[rstest]
async fn test_option_is_some_and_async(#[values(Some(1), None, Some(2))] value: Option<u32>) {
  let y = value.is_some_and(|val| val == 1);

  let ya = value
    .is_some_and_async(|val| async move {
      sleep(Duration::from_millis(100)).await;
      val == 1
    })
    .await;

  assert_eq!(y, ya);
}

#[rstest]
async fn test_option_map_async(#[values(Some(1), None, Some(2))] value: Option<u32>) {
  let y = value.map(|val| val + 1);
  let ya = value
    .map_async(|val| async move {
      sleep(Duration::from_millis(100)).await;
      val + 1
    })
    .await;

  assert_eq!(y, ya);
}
