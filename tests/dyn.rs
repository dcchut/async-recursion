use async_recursion::async_recursion;

#[async_recursion]
#[allow(dead_code)]
async fn dyn_coercion() -> Box<dyn core::fmt::Debug> {
    Box::new("hello world!")
}

#[async_recursion]
async fn no_return_type() {
    ()
}
