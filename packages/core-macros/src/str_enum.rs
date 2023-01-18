use std::env;

use napi_derive_backend::TypeDef;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemEnum, Lit, Visibility};

use super::write_to_typedef;

pub(super) fn str_enum_expand(item: ItemEnum) -> Result<TokenStream, syn::Error> {
    if let Visibility::Public(_) = item.vis {
        let enum_name = item.ident;
        let enum_pairs = item
            .variants
            .pairs()
            .map(|pair| pair.into_value())
            .map(|variant| {
                let expr = &variant
                    .discriminant
                    .as_ref()
                    .ok_or(syn::Error::new_spanned(
                        &variant.ident,
                        "Must have string value",
                    ))?
                    .1;
                let lit_expr = match expr {
                    Expr::Lit(lit_expr) => Ok(lit_expr),
                    _ => Err(syn::Error::new_spanned(&expr, "Must have string value")),
                }?;
                let lit_str = match &lit_expr.lit {
                    Lit::Str(lit_str) => Ok(lit_str),
                    _ => Err(syn::Error::new_spanned(&lit_expr, "Must have string value")),
                }?;
                Ok((&variant.ident, lit_str.value()))
            })
            .collect::<Result<Vec<_>, syn::Error>>()?;
        let (each_variant, each_str) = enum_pairs
            .iter()
            .map(|(id, val)| (id, val))
            .unzip::<_, _, Vec<&syn::Ident>, Vec<_>>();
        let invalid_str = format!("Invalid {} '{{}}'", enum_name.to_string());

        if let Ok(type_def_file) = env::var("TYPE_DEF_TMP_PATH") {
            let type_def = TypeDef {
                kind: "enum".to_owned(),
                def: enum_pairs
                    .iter()
                    .map(|(id, val)| format!("{} = '{}'", id.to_string(), val))
                    .collect::<Vec<_>>()
                    .join(",\n"),
                name: enum_name.to_string(),
                ..Default::default()
            };
            if let Err(err) = write_to_typedef(type_def_file, type_def) {
                println!("Failed to write type def file: {:?}", err);
            };
        };

        Ok(quote! {
            #[derive(Clone)]
            pub enum #enum_name {
                #(#each_variant),*
            }
            impl TryInto<#enum_name> for &String {
                type Error = ();

                fn try_into(self) -> Result<#enum_name, Self::Error> {
                    match self.as_str() {
                        #( #each_str => Ok(#enum_name::#each_variant), )*
                        _ => Err(()),
                    }
                }
            }
            impl From<#enum_name> for String {
                fn from(value: #enum_name) -> Self {
                    match value {
                        #( #enum_name::#each_variant => String::from(#each_str), )*
                    }
                }
            }
            impl napi::bindgen_prelude::ToNapiValue for #enum_name {
                // String already impls ToNapiValue, I trust String::to_napi_value is safe lol
                unsafe fn to_napi_value(
                    env: napi::sys::napi_env,
                    val: Self,
                ) -> napi::Result<napi::sys::napi_value> {
                    String::to_napi_value(env, String::from(val))
                }
            }
            impl napi::bindgen_prelude::FromNapiValue for #enum_name {
                // Should be safe
                unsafe fn from_napi_value(
                    env: napi::sys::napi_env,
                    napi_val: napi::sys::napi_value,
                ) -> napi::Result<Self> {
                    let value = String::from_napi_value(env, napi_val)?;
                    match (&value).try_into() as Result<#enum_name, _> {
                        Ok(variant) => Ok(variant),
                        Err(_) => Err(napi::Error::from_reason(format!(
                            #invalid_str,
                            value
                        ))),
                    }
                }
            }
        }
        .into())
    } else {
        return Err(syn::Error::new_spanned(&item, "Enum must be public"));
    }
}
