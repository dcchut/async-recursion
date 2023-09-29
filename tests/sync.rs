use std::future::pending;

use async_recursion::async_recursion;


#[async_recursion(Sync)]
async fn send_plus_sync() {
    pending::<()>().await;

}

#[async_recursion(?Send Sync)]
async fn only_sync() {
    pending::<()>().await;
}

#[async_recursion(?Send)]
async fn neither() {
    pending::<()>().await;

}