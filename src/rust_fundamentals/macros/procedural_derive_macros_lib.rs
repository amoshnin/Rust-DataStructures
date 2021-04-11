use proc_macro::TokenStream;
use quote::quote;
use syn;

// This function - Parses the `TokenStream`
#[proc_macro_derive(StylableMacro)]
pub fn stylable_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    return impl_stylable_trait(&ast);
}

// This function - Transforms the syntax tree
fn impl_stylable_trait(stream: &syn::DeriveInput) -> TokenStream {
    let name = &stream.ident;
    let gen = quote! {
        impl Stylable for #name {
            fn restyle() {
                println!("Hello, Macro! My name is {}", stringify!(#name))
            }
        }
    };
    return gen.into();
}