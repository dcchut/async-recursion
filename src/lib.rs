//! # Async Recursion Macro
//!
//! Procedural macro for recursive async functions.
//!
//! Consider the following recursive implementation of the fibonacci numbers:
//!
//! ```rust ignore
//! async fn fib(n : u32) -> u64 {
//!    match n {
//!        0     => panic!("zero is not a valid argument to fib()!"),
//!        1 | 2 => 1,
//!        3     => 2,
//!        _ => fib(n-1).await + fib(n-2).await
//!    }
//! }
//! ```
//!
//! The compiler helpfully tells us that:
//!
//! ```console
//! error[E0733]: recursion in an `async fn` requires boxing
//! --> src/main.rs:1:26
//!  |
//! 1 | async fn fib(n : u32) -> u64 {
//!  |                          ^^^ recursive `async fn`
//!  |
//!  = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`.
//! ```
//!
//! This crate provides an attribute macro to automatically convert an async function
//! to one returning a boxed Future.
//!
//! ## Example
//!
//! ```rust
//! use async_recursion::async_recursion;
//!
//! #[async_recursion]
//! async fn fib(n : u32) -> u64 {
//!    match n {
//!        0     => panic!("zero is not a valid argument to fib()!"),
//!        1 | 2 => 1,
//!        3     => 2,
//!        _ => fib(n-1).await + fib(n-2).await
//!    }
//! }
//! ```
//!
//! ## ?Send Option
//!
//! By default the returned future has a `Send` bound to make sure that it can be sent between threads. If this is not desired you can mark that you would like that that bound to be left out like so:
//!
//! ```rust ignore
//! #[async_recursion(?Send)]
//! async fn example() {}
//! ```
//!
//! In other words, `#[async_recursion]` modifies your function to return a [`BoxFuture`] and `#[async_recursion(?Send)]` modifies your function to return a [`LocalBoxFuture`].
//!
//! [`BoxFuture`]: https://docs.rs/futures/0.3.4/futures/future/type.BoxFuture.html
//! [`LocalBoxFuture`]: https://docs.rs/futures/0.3.4/futures/future/type.LocalBoxFuture.html
//!
//! ### License
//!
//! Licensed under either of
//!  * Apache License, Version 2.0
//!    ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license
//!    ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//! at your option.

extern crate proc_macro;

mod expand;
mod parse;

use crate::expand::expand;
use crate::parse::{AsyncItem, RecursionArgs};

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_recursion(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as AsyncItem);
    let args = parse_macro_input!(args as RecursionArgs);

    expand(&mut item, &args);

    TokenStream::from(quote!(#item))
}
