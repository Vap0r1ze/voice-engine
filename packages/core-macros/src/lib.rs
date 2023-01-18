use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Expr, ItemEnum, Lit, Visibility};

#[proc_macro_attribute]
pub fn str_enum(_attr: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(item) = parse::<ItemEnum>(input) {
        str_enum_expand(item).unwrap_or_else(|err| err.to_compile_error().into())
    } else {
        Default::default()
    }
}

fn str_enum_expand(item: ItemEnum) -> Result<TokenStream, syn::Error> {
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
        let variants = enum_pairs.iter().map(|pair| pair.0).collect::<Vec<_>>();
        let (try_into_arms, from_arms): (Vec<_>, Vec<_>) = enum_pairs
            .iter()
            .map(|pair| {
                let variant = pair.0;
                let value = &pair.1;
                (
                    quote! { #value => Ok(#enum_name::#variant) },
                    quote! { #enum_name::#variant => String::from(#value) },
                )
            })
            .unzip();
        let invalid_str = format!("Invalid {} '{{}}'", enum_name.to_string());

        Ok(quote! {
            #[derive(Clone)]
            pub enum #enum_name {
                #(#variants),*
            }
            impl TryInto<#enum_name> for &String {
                type Error = ();

                fn try_into(self) -> Result<#enum_name, Self::Error> {
                    match self.as_str() {
                        #(#try_into_arms,)*
                        _ => Err(()),
                    }
                }
            }
            impl From<#enum_name> for String {
                fn from(value: #enum_name) -> Self {
                    match value {
                        #(#from_arms,)*
                    }
                }
            }
            impl ToNapiValue for #enum_name {
                // String already impls ToNapiValue, I trust String::to_napi_value is safe lol
                unsafe fn to_napi_value(
                    env: napi::sys::napi_env,
                    val: Self,
                ) -> napi::Result<napi::sys::napi_value> {
                    String::to_napi_value(env, String::from(val))
                }
            }

            impl FromNapiValue for #enum_name {
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
