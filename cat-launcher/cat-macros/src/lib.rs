extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// A procedural macro to derive `serde::Serialize` for command error types.
///
/// This macro generates a `Serialize` implementation that includes the error's
/// type (from `Into<&'static str>`) and its message (from `ToString`).
#[proc_macro_derive(CommandErrorSerialize)]
pub fn derive_command_error_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let name_str = name.to_string();
    let Data::Enum(data_enum) = &input.data else {
        return syn::Error::new_spanned(
            name,
            "CommandErrorSerialize can only be derived for enums",
        )
        .to_compile_error()
        .into();
    };

    let variant_arms = data_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_str = variant_name.to_string();
        match &variant.fields {
            Fields::Unit => quote! {
                Self::#variant_name => #variant_str,
            },
            Fields::Unnamed(_) => quote! {
                Self::#variant_name(..) => #variant_str,
            },
            Fields::Named(_) => quote! {
                Self::#variant_name { .. } => #variant_str,
            },
        }
    });

    let expanded = quote! {
        impl ::serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeStruct;
                let mut st = serializer.serialize_struct(#name_str, 2)?;

                let err_type: &'static str = match self {
                    #(#variant_arms)*
                };
                st.serialize_field("type", &err_type)?;

                let msg = self.to_string();
                st.serialize_field("message", &msg)?;

                st.end()
            }
        }
    };

    TokenStream::from(expanded)
}
