use crate::crypt::*;
use napi_derive::napi;

#[napi(constructor)]
#[derive(Clone)]
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
impl VoiceConnection {
    #[napi]
    pub fn get_ip(&self) -> napi::Result<Vec<u8>> {
        self.options
            .address
            .split('.')
            .into_iter()
            .map(|s| s.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| napi::Error::from_reason("bad ip"))
    }
}
