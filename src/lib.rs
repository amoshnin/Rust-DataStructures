use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

// --> Builder Pattern for a Type Custom Derive Procedural Macro
#[proc_macro_derive(Builder)]
pub fn builder_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", ast);
    impl_builder_trait(&ast)
}

fn impl_builder_trait(_stream: &DeriveInput) -> TokenStream {
    TokenStream::new()
}

// --> Reference Custom Derive Procedural Macro Example
// This function - Parses the `TokenStream`
#[proc_macro_derive(StylableMacro)]
pub fn stylable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree, that we can manipulate
    let ast = parse_macro_input!(input as DeriveInput);
    // Build the trait implementation
    impl_stylable_trait(&ast)
}

// This function - Transforms the syntax tree
fn impl_stylable_trait(stream: &DeriveInput) -> TokenStream {
    let name = &stream.ident;
    let gen = quote! {
        impl Stylable for #name {
            fn restyle() -> String {
                return format!("{}", stringify!(#name));
            }
        }
    };
    gen.into()
}
