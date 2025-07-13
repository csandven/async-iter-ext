use std::time::Duration;

use async_iter_ext::AsyncResultTools;
use async_std::task::sleep;
use rstest::rstest;

#[derive(Debug, PartialEq)]
enum Error {
    Invalid,
}

async fn is_even(x: u32) -> bool {
    x % 2 == 0
}

async fn double(x: u32) -> u32 {
    x * 2
}

async fn double_ok(x: u32) -> Result<u32, &'static str> {
    Ok(x * 2)
}

async fn err_to_uppercase(e: &'static str) -> &'static str {
    match e {
        "fail" => "FAIL",
        other => other,
    }
}

async fn recover(e: &'static str) -> Result<u32, &'static str> {
    if e == "recover" {
        Ok(99)
    } else {
        Err(e)
    }
}

#[rstest]
#[case(Ok(4), true)]
#[case(Ok(5), false)]
#[case(Err("oops"), false)]
async fn test_is_ok_and_async(#[case] input: Result<u32, &str>, #[case] expected: bool) {
    let actual =  input.is_ok_and_async(is_even).await;
    assert_eq!(actual, expected);
}

#[rstest]
#[case(Ok(3), Ok(6))]
#[case(Err("fail"), Err("fail"))]
async fn test_map_async(#[case] input: Result<u32, &str>, #[case] expected: Result<u32, &str>) {
    let actual = input.map_async(double).await;
    assert_eq!(actual, expected);
}

#[rstest]
#[case(Ok(3), Ok(6))]
#[case(Err("fail"), Err("fail"))]
async fn test_and_then_async(#[case] input: Result<u32, &str>, #[case] expected: Result<u32, &str>) {
    let actual =  input.and_then_async(double_ok).await;
    assert_eq!(actual, expected);
}

#[rstest]
#[case(Ok(7), Ok(7))]
#[case(Err("fail"), Err("FAIL"))]
#[case(Err("recover"), Err("recover"))]
async fn test_map_err_async(#[case] input: Result<u32, &str>, #[case] expected: Result<u32, &str>) {
    let actual = input.map_err_async(err_to_uppercase).await;
    assert_eq!(actual, expected);
}

#[rstest]
#[case(Ok(10), Ok(10))]
#[case(Err("recover"), Ok(99))]
#[case(Err("fail"), Err("fail"))]
async fn test_or_else_async(#[case] input: Result<u32, &str>, #[case] expected: Result<u32, &str>) {
    let actual = input.or_else_async(recover).await;
    assert_eq!(actual, expected);
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
