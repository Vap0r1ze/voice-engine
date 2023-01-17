use napi_derive::napi;

use crate::crypt::*;

#[napi(constructor)]
pub struct VoiceConnection {
    pub user_id: String,
    pub options: ConnectionOptions,
}

#[napi(object)]
#[derive(Clone)]
pub struct ConnectionOptions {
    pub address: String,
    pub port: u16,
    pub ssrc: u32,
    pub modes: Vec<CipherMode>,
    pub stream_params: Vec<StreamParameters>,
    pub stream_user_id: Option<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct StreamParameters {
    pub user_id: String,
    pub address: String,
    pub port: u16,
    pub ssrc: u32,
    pub modes: Vec<CipherMode>,
}

#[napi]
pub fn _create_voice_connection(user_id: String, options: ConnectionOptions) -> VoiceConnection {
    VoiceConnection {
        user_id,
        options: ConnectionOptions {
            address: options.address,
            port: options.port,
            ssrc: options.ssrc,
            modes: options.modes,
            stream_params: options.stream_params,
            stream_user_id: options.stream_user_id,
        },
    }
}
