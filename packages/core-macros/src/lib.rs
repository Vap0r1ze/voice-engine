use std::io::Write;
use std::{fs, io::BufWriter};

use napi_derive_backend::TypeDef;
use proc_macro::TokenStream;
use syn::{parse, ItemEnum};

mod napi_strum;

#[proc_macro_attribute]
pub fn napi_strum(_attr: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(item) = parse::<ItemEnum>(input) {
        napi_strum::expand(item).unwrap_or_else(|err| err.to_compile_error().into())
    } else {
        Default::default()
    }
}

fn write_to_typedef(path: String, type_def: TypeDef) -> Result<(), std::io::Error> {
    let file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    let mut writer = BufWriter::<fs::File>::new(file);
    writer.write_all(type_def.to_string().as_bytes())?;
    writer.write_all("\n".as_bytes())?;
    Ok(())
}
