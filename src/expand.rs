use crate::parse::AsyncItem;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{parse_quote, Signature, Block, ReturnType};

impl ToTokens for AsyncItem {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

pub fn expand(
    item : &mut AsyncItem
) {
    transform_sig(&mut item.0.sig);
    transform_block(&mut item.0.block);
}

fn transform_block(
    block: &mut Block
) {
    let brace = block.brace_token;
    *block = parse_quote!({
        Box::pin(async move #block)
    });
    block.brace_token = brace;
    return;
}


// Input:
//     async fn f<S, T>(x : S, y : &T) -> Ret;
//
// Output:
//     fn f<S, T>(x : S, y : &T) -> Pin<Box<dyn Future<Output = Ret> + Send>
fn transform_sig(
    sig: &mut Signature
) {
    // Determine the original return type
    let ret = match &sig.output {
        ReturnType::Default => quote!(()),
        ReturnType::Type(_, ret) => quote!(#ret),
    };

    // Remove the asyncness of this function
    sig.asyncness = None;

    // Modify the return type
    sig.output = parse_quote! {
        -> core::pin::Pin<Box<
            dyn core::future::Future<Output = #ret>>>
    };
}