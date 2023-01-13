export type CipherMode = // NOTE: there may be more ciphers than this

    | "aead_aes256_gcm"
    | "aead_aes256_gcm_rtpsize"
    | "aead_xchacha20_poly1305_rtpsize"
    | "xsalsa20_poly1305"
    | "xsalsa20_poly1305_lite"
    | "xsalsa20_poly1305_suffix"
    | "xsalsa20_poly1305_lite_rtpsize";
