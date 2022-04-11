extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;


#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive( input: TokenStream ) -> TokenStream {
    // construct a representation fo Rust code as a syntax tree
    // that we can then manipulate
    let ast = syn::parse( input ).unwrap();

    // build the trai implementation
    impl_hello_macro( &ast )

}


fn impl_hello_macro( ast: &syn::DeriveInput ) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!( "Hello, Macro! My name is {}!", stringify!( #name ) );
            }
        }
    };

    gen.into()
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
