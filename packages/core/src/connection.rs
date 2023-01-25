use std::{sync::mpsc, thread};

use crate::{audio::AudioManager, crypt::*, transport::ConnectionTransportOptions};
use napi_derive::napi;

#[napi]
pub struct VoiceConnectionHandle {
    pub user_id: String,
    pub options: ConnectionOptions,
    thread_join: thread::JoinHandle<()>,
    transport_tx: mpsc::Sender<ConnectionTransportOptions>,
}

#[napi(object)]
#[derive(Clone, Default)]
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
impl VoiceConnectionHandle {
    pub fn new(user_id: String, options: ConnectionOptions) -> Self {
        let (transport_tx, transport_rx) = mpsc::channel();

        let user_id_2 = user_id.clone();
        let options_2 = options.clone();
        let thread_join = thread::spawn(move || {
            let mut connection = VoiceConnection::new(user_id_2, options_2, transport_rx);
            connection.start();
        });

        Self {
            user_id,
            options,
            thread_join,
            transport_tx,
        }
    }

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

    #[napi]
    pub fn set_transport_options(
        &mut self,
        options: ConnectionTransportOptions,
    ) -> napi::Result<()> {
        self.transport_tx.send(options).map_err(|_| {
            napi::Error::from_reason("Could not send transport options as channel is closed")
        })
    }
}

pub struct VoiceConnection {
    user_id: String,
    options: ConnectionOptions,
    transport_opts: ConnectionTransportOptions,
    transport_rx: mpsc::Receiver<ConnectionTransportOptions>,
}

impl VoiceConnection {
    pub fn new(
        user_id: String,
        options: ConnectionOptions,
        transport_rx: mpsc::Receiver<ConnectionTransportOptions>,
    ) -> Self {
        Self {
            user_id,
            options,
            transport_opts: ConnectionTransportOptions::default(),
            transport_rx,
        }
    }

    pub fn start(&mut self) {
        loop {
            for transport_opts in self.transport_rx.try_iter() {
                self.transport_opts.merge(transport_opts);
            }
        }
    }
}
