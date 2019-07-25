extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput};

#[proc_macro_derive(Visit)]
pub fn visit_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse_macro_input!(input);
    let name = input.ident;

    let walk_impl = match input.data {
        Data::Struct(data) => struct_walk_impl(data),
        Data::Enum(data) => enum_walk_impl(data),
        Data::Union(_) => unimplemented!(),
    };

    let tokens = quote! {
        use tourism::{Visit, Visitor};

        impl<W> Visit<W> for #name where W: Visitor<Self> + ?Sized {
            fn walk(&mut self, visitor: &mut W) {
                #walk_impl
            }
        }
    };

    tokens.into()
}

fn struct_walk_impl(data: DataStruct) -> proc_macro2::TokenStream {
    unimplemented!()
}

fn enum_walk_impl(data: DataEnum) -> proc_macro2::TokenStream {
    unimplemented!()
}
