use async_recursion::async_recursion;
struct Node<'a, T> {
    value: T,
    next: Option<&'a Node<'a, T>>,
}
async fn contains_value<'a, T: PartialEq>(value: &T, node: &Node<'a, T>) -> bool {
    &node.value == value
}
fn contains_value_2<'a, 'b, 'async_recursion, T: PartialEq>(
    value: &'b T,
    node: &'b Node<'a, T>,
) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = bool> + 'async_recursion>>
where
    T: 'async_recursion,
    'b: 'async_recursion,
{
    Box::pin(async move { contains_value(value, node).await })
}
