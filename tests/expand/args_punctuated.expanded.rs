use async_recursion::async_recursion;
fn not_send_sync_1() -> ::core::pin::Pin<
    Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Sync>,
> {
    Box::pin(async move {})
}
fn not_send_sync_2() -> ::core::pin::Pin<
    Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Sync>,
> {
    Box::pin(async move {})
}
fn sync_not_send_1() -> ::core::pin::Pin<
    Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Sync>,
> {
    Box::pin(async move {})
}
fn sync_not_send_2() -> ::core::pin::Pin<
    Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Sync>,
> {
    Box::pin(async move {})
}
