use std::{sync::mpsc, thread};

use crate::{crypt::*, transport::ConnectionTransportOptions};
use napi_derive::napi;

#[napi]
pub struct VoiceConnectionHandle {
    pub user_id: String,
    pub settings: ConnectionSettings,
    thread_join: Option<thread::JoinHandle<()>>,
    request_tx: crossbeam_channel::Sender<VoiceRequest>,
    event_rx: crossbeam_channel::Receiver<VoiceEvent>,
}

#[napi(object)]
#[derive(Clone, Default)]
pub struct ConnectionSettings {
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

pub enum VoiceRequest {
    Options(Box<ConnectionTransportOptions>),
    Destroy,
}
pub enum VoiceEvent {
    //
}

#[napi]
impl VoiceConnectionHandle {
    pub fn new(user_id: String, settings: ConnectionSettings) -> Self {
        let (event_tx, event_rx) = crossbeam_channel::unbounded();
        let (request_tx, request_rx) = crossbeam_channel::unbounded();

        let user_id_2 = user_id.clone();
        let settings_2 = settings.clone();
        let thread_join = thread::spawn(move || {
            let mut connection = VoiceConnection {
                user_id: user_id_2,
                settings: settings_2,
                options: Default::default(),
                request_rx,
                event_tx,
            };
            connection.do_cycle();
        });

        Self {
            user_id,
            settings,
            thread_join: Some(thread_join),
            event_rx,
            request_tx,
        }
    }

    #[napi]
    pub fn destroy(&mut self) {
        _ = self.request_tx.send(VoiceRequest::Destroy);
        if let Some(thread_join) = self.thread_join.take() {
            _ = thread_join.join();
        }
    }

    #[napi]
    pub fn get_ip(&self) -> napi::Result<Vec<u8>> {
        self.settings
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
        self.request_tx
            .send(VoiceRequest::Options(Box::new(options)))
            .map_err(|_| {
                napi::Error::from_reason("Could not send transport options as channel is closed")
            })
    }
}

pub struct VoiceConnection {
    user_id: String,
    settings: ConnectionSettings,
    options: ConnectionTransportOptions,
    request_rx: crossbeam_channel::Receiver<VoiceRequest>,
    event_tx: crossbeam_channel::Sender<VoiceEvent>,
}

impl VoiceConnection {
    pub fn do_cycle(&mut self) {
        loop {
            // Handle requests from napi thread
            for request in self.request_rx.try_iter() {
                match request {
                    VoiceRequest::Destroy => {
                        return;
                    }
                    VoiceRequest::Options(options) => {
                        self.options.merge(*options);
                    }
                }
            }
        }
    }
}
