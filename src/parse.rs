use proc_macro2::Span;
use syn::{
    parse::{Error, Parse, ParseStream, Result},
    token::Question,
    ItemFn, Token,
};

pub struct AsyncItem(pub ItemFn);

impl Parse for AsyncItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let item: ItemFn = input.parse()?;

        // Check that this is an async function
        if item.sig.asyncness.is_none() {
            return Err(Error::new(Span::call_site(), "expected an async function"));
        }

        Ok(AsyncItem(item))
    }
}

pub struct RecursionArgs {
    pub send_bound: bool,
    pub sync_bound: bool,
}

impl Default for RecursionArgs {
    fn default() -> Self {
        RecursionArgs {
            send_bound: true,
            sync_bound: false,
        }
    }
}

/// Custom keywords for parser
mod kw {
    syn::custom_keyword!(Send);
    syn::custom_keyword!(Sync);
}

impl Parse for RecursionArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let Self {
            mut send_bound,
            mut sync_bound,
        } = Self::default();
        // Check for the `?Send` option
        if input.peek(Token![?]) {
            input.parse::<Question>()?;
            input.parse::<kw::Send>()?;
            send_bound = false;
        }
        if input.peek(kw::Sync) {
            input.parse::<kw::Sync>()?;
            sync_bound = true;
        }

        if !input.is_empty() {
            Err(input.error("expected `?Send` or empty"))
        } else {
            Ok(Self {
                send_bound,
                sync_bound,
            })
        }
    }
}
