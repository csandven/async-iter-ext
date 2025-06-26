use std::time::Duration;

use async_iter_ext::{AsyncIterTools, iter::process_result::ProcessResultsStrategy};
use async_std::task::sleep;
use rstest::rstest;

#[rstest]
async fn test_process_results_default_strategy_only_success() {
  let items = [1, 2, 3, 4];

  let res = items
    .into_iter()
    .map_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      Ok(item * 2)
    })
    .process_results::<_, &str>()
    .await;

  assert!(res.errors().is_empty());
  assert_eq!(res.into_successes(), vec![2, 4, 6, 8]);
}

#[rstest]
async fn test_process_results_default_strategy_success_and_errors() {
  let items = [1, 2, 3, 4];

  let res = items
    .into_iter()
    .map_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      if item > 2 {
        Err("Item was greater than 2")
      } else {
        Ok(item * 2)
      }
    })
    .process_results::<_, &str>()
    .await;

  assert_eq!(res.clone().into_successes(), vec![2, 4]);
  assert_eq!(res.clone().into_errors(), vec!["Item was greater than 2", "Item was greater than 2"]);
}

#[rstest]
async fn test_process_results_break_on_error_strategy_only_success() {
  let items = [1, 2, 3, 4];

  let res = items
    .into_iter()
    .map_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      Ok(item * 2)
    })
    .process_results::<_, &str>()
    .with_process_strategy(ProcessResultsStrategy::BreakOnError)
    .await;

  assert!(res.errors().is_empty());
  assert_eq!(res.into_successes(), vec![2, 4, 6, 8]);
}

#[rstest]
async fn test_process_results_break_on_error_strategy_success_and_errors() {
  let items = [1, 2, 3, 4];

  let res = items
    .into_iter()
    .map_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      if item > 2 {
        Err("Item was greater than 2")
      } else {
        Ok(item * 2)
      }
    })
    .process_results::<_, &str>()
    .with_process_strategy(ProcessResultsStrategy::BreakOnError)
    .await;

  assert!(res.is_empty());
  assert_eq!(res.into_errors(), vec!["Item was greater than 2"]);
}

#[rstest]
async fn test_process_results_break_on_error_strategy_only_success_into_result() {
  let items = [1, 2, 3, 4];

  let res = items
    .into_iter()
    .map_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      Ok(item * 2)
    })
    .process_results::<_, &str>()
    .with_process_strategy(ProcessResultsStrategy::BreakOnError)
    .await
    .into_result();

  assert!(res.is_ok_and(|items| items == vec![2, 4, 6, 8]));
}

#[rstest]
async fn test_process_results_break_on_error_strategy_success_and_errors_into_result() {
  let items = [1, 2, 3, 4];

  let res = items
    .into_iter()
    .map_async(|item| async move {
      sleep(Duration::from_millis(100)).await;
      if item > 2 {
        Err("Item was greater than 2")
      } else {
        Ok(item * 2)
      }
    })
    .process_results::<_, &str>()
    .with_process_strategy(ProcessResultsStrategy::BreakOnError)
    .await
    .into_result();

  assert!(res.is_err_and(|err| err == "Item was greater than 2"));
}
