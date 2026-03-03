use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

extern crate proc_macro;

#[proc_macro_derive(Parameterized, attributes(parameter))]
pub fn derive_parameterized(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut param_entries: Vec<_> = Vec::new();
    let mut param_updates: Vec<_> = Vec::new();
    #[allow(clippy::collapsible_if)]
    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(fields) = &data_struct.fields {
            for field in fields.named.iter() {
                let field_name = &field.ident;
                let mut param_name = field_name.as_ref().unwrap().to_string();
                let mut include = false;

                for attr in field
                    .attrs
                    .iter()
                    .filter(|attr| attr.path().is_ident("parameter"))
                {
                    include = true;
                    let _ = attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("name") {
                            let lit: syn::LitStr = meta.value()?.parse()?;
                            param_name = lit.value();
                        }
                        Ok(())
                    });
                }

                if include {
                    param_entries.push(quote! {
                        parameters.push(#param_name.to_string(), ::force_smith::prelude::ToParameter::to_parameter(&self.#field_name));
                    });
                    param_updates.push(quote! {
                        if let Some(val) = parameters.iter().find(|(k, _)| k == #param_name).and_then(|(_, param)| <_ as ::force_smith::prelude::FromParameter>::from_parameter(param)) {
                            self.#field_name = val;
                        }
                    });
                }
            }
        }
    }

    let expanded = quote! {
        impl ::force_smith::prelude::Parameterized for #struct_name {
            fn get_parameters(&self) -> Vec<(String, ::force_smith::prelude::Parameter)> {
                let mut parameters = Vec::new();
                #(#param_entries)*
                parameters
            }

            fn update_parameters(&mut self, parameters: &Vec<(String, ::force_smith::prelude::Parameter)>) {
                #(#param_updates)*
            }
        }
    };
    TokenStream::from(expanded)
}
