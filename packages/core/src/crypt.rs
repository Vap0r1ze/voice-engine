use napi::bindgen_prelude::{FromNapiValue, ToNapiValue};

#[derive(Clone)]
pub enum CipherMode {
    AeadAes256Gcm,
    AeadAes256GcmRtpsize,
    AeadXchacha20Poly1305Rtpsize,
    Xsalsa20Poly1305,
    Xsalsa20Poly1305Lite,
    Xsalsa20Poly1305Suffix,
    Xsalsa20Poly1305LiteRtpsize,
}

impl TryInto<CipherMode> for &String {
    type Error = ();

    fn try_into(self) -> Result<CipherMode, Self::Error> {
        match self.as_str() {
            "aead_aes256_gcm" => Ok(CipherMode::AeadAes256Gcm),
            "aead_aes256_gcm_rtpsize" => Ok(CipherMode::AeadAes256GcmRtpsize),
            "aead_xchacha20_poly1305_rtpsize" => Ok(CipherMode::AeadXchacha20Poly1305Rtpsize),
            "xsalsa20_poly1305" => Ok(CipherMode::Xsalsa20Poly1305),
            "xsalsa20_poly1305_lite" => Ok(CipherMode::Xsalsa20Poly1305Lite),
            "xsalsa20_poly1305_suffix" => Ok(CipherMode::Xsalsa20Poly1305Suffix),
            "xsalsa20_poly1305_lite_rtpsize" => Ok(CipherMode::Xsalsa20Poly1305LiteRtpsize),
            _ => Err(()),
        }
    }
}

impl From<CipherMode> for String {
    fn from(value: CipherMode) -> Self {
        match value {
            CipherMode::AeadAes256Gcm => String::from("aead_aes256_gcm"),
            CipherMode::AeadAes256GcmRtpsize => String::from("aead_aes256_gcm_rtpsize"),
            CipherMode::AeadXchacha20Poly1305Rtpsize => {
                String::from("aead_xchacha20_poly1305_rtpsize")
            }
            CipherMode::Xsalsa20Poly1305 => String::from("xsalsa20_poly1305"),
            CipherMode::Xsalsa20Poly1305Lite => String::from("xsalsa20_poly1305_lite"),
            CipherMode::Xsalsa20Poly1305Suffix => String::from("xsalsa20_poly1305_suffix"),
            CipherMode::Xsalsa20Poly1305LiteRtpsize => {
                String::from("xsalsa20_poly1305_lite_rtpsize")
            }
        }
    }
}

impl ToNapiValue for CipherMode {
    // String already impls ToNapiValue, I trust String::to_napi_value is safe lol
    unsafe fn to_napi_value(
        env: napi::sys::napi_env,
        val: Self,
    ) -> napi::Result<napi::sys::napi_value> {
        String::to_napi_value(env, String::from(val))
    }
}

impl FromNapiValue for CipherMode {
    // Should be safe
    unsafe fn from_napi_value(
        env: napi::sys::napi_env,
        napi_val: napi::sys::napi_value,
    ) -> napi::Result<Self> {
        let value = String::from_napi_value(env, napi_val)?;
        match (&value).try_into() as Result<CipherMode, _> {
            Ok(mode) => Ok(mode),
            Err(_) => Err(napi::Error::from_reason(format!(
                "Invalid CipherMode '{}'",
                value
            ))),
        }
    }
}
