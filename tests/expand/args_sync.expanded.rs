use async_recursion::async_recursion;
fn sync() -> ::core::pin::Pin<
    Box<
        dyn ::core::future::Future<
            Output = (),
        > + ::core::marker::Send + ::core::marker::Sync,
    >,
> {
    Box::pin(async move {})
}
