extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CodedError)]
pub fn coded_error_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                use strum::IntoStaticStr;

                let mut st = serializer.serialize_struct(stringify!(#name), 2)?;

                let err_type: &'static str = self.into();
                st.serialize_field("type", &err_type)?;

                let msg = self.to_string();
                st.serialize_field("message", &msg)?;

                st.end()
            }
        }
    };

    TokenStream::from(expanded)
}
