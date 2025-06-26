use std::time::Duration;

use async_iter_ext::AsyncResultTools;
use async_std::task::sleep;
use rstest::rstest;

#[derive(Debug, PartialEq)]
enum Error {
    Invalid,
}

#[rstest]
async fn test_result_is_ok_and_async(#[values(Some(1), None, Some(2))] value: Option<u32>) {
    let y = value.ok_or(Error::Invalid).is_ok_and(|val| val == 1);

    let ya = value
        .ok_or(Error::Invalid)
        .is_ok_and_async(|val| async move {
            sleep(Duration::from_millis(100)).await;
            val == 1
        })
        .await;

    assert_eq!(y, ya);
}

#[rstest]
async fn test_result_map_async(#[values(Some(1), None, Some(2))] value: Option<u32>) {
    let y = value.ok_or(Error::Invalid).map(|val| val + 1);
    let ya = value
        .ok_or(Error::Invalid)
        .map_async(|val| async move {
            sleep(Duration::from_millis(100)).await;
            val + 1
        })
        .await;

    assert_eq!(y, ya);
}
