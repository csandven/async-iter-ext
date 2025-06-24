use std::time::Duration;

use async_iter_ext::AsyncIterTools;
use async_std::task::sleep;

#[rstest::rstest]
async fn test_for_each_async_mut() {
  let items = [1, 2, 3, 4];

  let mut mut_items = items;
  mut_items
    .iter_mut()
    .for_each_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      *item += 3;
    })
    .await;

  assert_eq!(mut_items.len(), items.len());
  assert_eq!(mut_items, [4, 5, 6, 7]);
}
