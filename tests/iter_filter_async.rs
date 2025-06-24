use std::time::Duration;

use async_iter_ext::{AsyncIterTools, iter::AsyncIterator};
use async_std::task::sleep;
use rstest::rstest;

#[rstest]
async fn test_simple_filter() {
  let items = vec![1, 2, 3];

  let filtered_items = items
    .into_iter()
    .filter_async(|i| async move {
      sleep(Duration::from_millis(100)).await;
      i == 2
    })
    .async_collect::<Vec<_>>()
    .await;

  assert_eq!(filtered_items, vec![2]);
}

#[rstest]
async fn test_multiple_filters() {
  let items = vec![1, 2, 3];

  let filtered_items = items
    .into_iter()
    .filter_async(|i| async move {
      sleep(Duration::from_millis(100)).await;
      i > 1
    })
    .filter_async(|i| async move {
      sleep(Duration::from_millis(100)).await;
      i < 3
    })
    .async_collect::<Vec<_>>()
    .await;

  assert_eq!(filtered_items, vec![2]);
}

#[rstest]
#[timeout(Duration::from_millis(400))]
async fn test_async_filter_then_sync_filter() {
  let items = vec![1, 2, 3];

  let filtered_items = items
    .into_iter()
    .filter_async(|i| async move {
      sleep(Duration::from_millis(100)).await;
      i > 1
    })
    .await
    .filter(|i| *i < 3)
    .collect::<Vec<_>>();

  assert_eq!(filtered_items, vec![2]);
}
