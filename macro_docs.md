# Example
```rust
use async_recursion::async_recursion;

#[async_recursion]
async fn fib(n: u32) -> u64 {
    match n {
        0 => panic!("zero is not a valid argument to fib()!"),
        1 | 2 => 1,
        _ => fib(n-1).await + fib(n-2).await
    }
}
```

## ?Send Option

By default the returned future has a [`Send`] bound to make sure it can be sent between
threads.  If this is undesirable you can mark that the bound should be left out like so:

```
# use async_recursion::async_recursion;

#[async_recursion(?Send)]
async fn example() {
    // ...
}
```

In detail:

- `#[async_recursion]` modifies your function to return a [`BoxFuture`], and
- `#[async_recursion(?Send)]` modifies your function to return a [`LocalBoxFuture`].

[`BoxFuture`]: https://docs.rs/futures/0.3.4/futures/future/type.BoxFuture.html
[`LocalBoxFuture`]: https://docs.rs/futures/0.3.4/futures/future/type.LocalBoxFuture.html