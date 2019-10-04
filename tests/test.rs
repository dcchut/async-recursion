use async_recursion::async_recursion;
use futures_executor::block_on;

#[async_recursion]
#[doc("TESTING 12345")]
async fn fib(n : u32) -> u64 {
    match n {
        0     => panic!("zero is not a valid argument to fib()!"),
        1 | 2 => 1,
        3     => 2,
        _ => fib(n-1).await + fib(n-2).await
    }
}

#[test]
fn fibonacci_works() {
    assert_eq!(block_on(fib(3)), 2);
    assert_eq!(block_on(fib(4)), 3);
    assert_eq!(block_on(fib(5)), 5);
}