use std::{ops::Mul, time::Duration};

use async_iter_ext::{AsyncIterTools, iter::AsyncIterator};
use async_std::task::sleep;
use rstest::rstest;

async fn multiply_item<T: Mul<i32, Output = i32>>(value: T) -> i32 {
  sleep(Duration::from_millis(100)).await;
  value * 2
}

#[rstest]
async fn test_single_multiply() {
  let items = [1, 2, 3, 4];

  let mapped_items_vec = items.iter().map_async(multiply_item).async_collect::<Vec<_>>().await;

  assert_eq!(mapped_items_vec.len(), items.len());
  assert_eq!(mapped_items_vec, vec![2, 4, 6, 8]);
}

#[rstest]
async fn test_multiple_multiply() {
  let items = [1, 2, 3, 4];

  let mapped_items_vec = items
    .iter()
    .map_async(multiply_item)
    .map_async(multiply_item)
    .map_async(multiply_item)
    .async_collect::<Vec<_>>()
    .await;

  assert_eq!(mapped_items_vec.len(), items.len());
  assert_eq!(mapped_items_vec, vec![8, 16, 24, 32]);
}

#[rstest]
#[timeout(Duration::from_millis(400))]
async fn test_async_map_then_sync_map() {
  let items = [1, 2, 3, 4];

  let async_and_then_sync = items
    .iter()
    .map_async(multiply_item)
    .await
    .map(|item| item * 2)
    .collect::<Vec<_>>();

  assert_eq!(async_and_then_sync.len(), items.len());
  assert_eq!(async_and_then_sync, vec![4, 8, 12, 16]);
}
