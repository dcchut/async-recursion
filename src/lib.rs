//! <h5>Procedural macro for recursive async functions</h5>
//!
//! Consider the following recursive implementation of the fibonacci numbers:
//!
//! ```compile_fail
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
//! ```text
//! error[E0733]: recursion in an `async fn` requires boxing
//! --> src/main.rs:1:26
//!  |
//! 1 | async fn fib(n : u32) -> u64 {
//!  |                          ^^^ recursive `async fn`
//!  |
//!  = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`.
//! ```
//!
//! This crate provides an attribute macro to automatically convert recursive async functions
//! to functions returning a Box<dyn Future>.
//!
//! # Example
//!
//! ```
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

extern crate proc_macro;

mod parse;
mod expand;

use crate::parse::AsyncItem;
use crate::expand::expand;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

#[proc_macro_attribute]
pub fn async_recursion(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as AsyncItem);

    // Transform the function signature
    expand(&mut item);

    TokenStream::from(quote!(#item))
}