use core_macros::napi_strum;

#[napi_strum]
pub enum CipherMode {
    AeadAes256Gcm = "aead_aes256_gcm",
    AeadAes256GcmRtpsize = "aead_aes256_gcm_rtpsize",
    AeadXchacha20Poly1305Rtpsize = "aead_xchacha20_poly1305_rtpsize",
    Xsalsa20Poly1305 = "xsalsa20_poly1305",
    Xsalsa20Poly1305Lite = "xsalsa20_poly1305_lite",
    Xsalsa20Poly1305Suffix = "xsalsa20_poly1305_suffix",
    Xsalsa20Poly1305LiteRtpsize = "xsalsa20_poly1305_lite_rtpsize",
}
