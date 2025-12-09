extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(CommandErrorSerialize)]
pub fn derive_command_error_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let name_str = name.to_string();

    let expanded = quote! {
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
                for<'a> &'a Self: Into<&'static str>,
            {
                use ::serde::ser::SerializeStruct;
                let mut st = serializer.serialize_struct(#name_str, 2)?;

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
