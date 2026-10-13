use async_recursion::async_recursion;

struct Node<'a, T> {
    value: T,
    next: Option<&'a Node<'a, T>>,
}

async fn contains_value<'a, T: PartialEq>(value: &T, node: &Node<'a, T>) -> bool {
    &node.value == value
}

#[async_recursion(?Send)]
async fn contains_value_2<'a, 'b, T: PartialEq>(value: &'b T, node: &'b Node<'a, T>) -> bool {
    contains_value(value, node).await
}