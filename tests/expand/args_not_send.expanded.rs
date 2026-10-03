use async_recursion::async_recursion;
fn no_send_bound() -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()>>> {
    Box::pin(async move {})
}
