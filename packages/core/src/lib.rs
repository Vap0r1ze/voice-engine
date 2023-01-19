#![deny(clippy::all)]

use std::collections::HashMap;

use callbacks::{CallbackRef, CoreCallbackStore};
use connection::{ConnectionOptions, VoiceConnection};
use napi::{Env, JsFunction, JsNumber};
use napi_derive::napi;
use transport::CoreTransportOptions;

mod callbacks;
mod connection;
mod crypt;
mod transport;

#[napi]
#[derive(Default)]
pub struct VoiceCore {
    callbacks: CoreCallbackStore,
    connections: HashMap<(String, u32), VoiceConnection>,
    transport_opts: CoreTransportOptions,
}

#[napi]
fn _start() -> VoiceCore {
    Default::default()
}

define_callback!(VoiceCore, set_device_change_callback, device_change);
define_callback!(VoiceCore, set_volume_change_callback, volume_change);
define_callback!(
    VoiceCore,
    set_video_input_initialization_callback,
    video_input_init
);

#[napi]
impl VoiceCore {
    #[napi]
    pub fn set_local_volume(&self, env: Env, volume: JsNumber) -> napi::Result<()> {
        if let Some(reference) = &self.callbacks.volume_change {
            let format = env.create_string("Volume was set to %o, also six = %o")?;
            let six = env.create_int32(6)?;
            reference.call(
                None,
                &[
                    format.into_unknown(),
                    volume.into_unknown(),
                    six.into_unknown(),
                ],
            )?;
        } else {
        };
        Ok(())
    }

    #[napi]
    pub fn create_voice_connection(
        &mut self,
        user_id: String,
        options: ConnectionOptions,
    ) -> VoiceConnection {
        let conn = VoiceConnection {
            user_id: user_id.to_string(),
            options: options.clone(),
            ..Default::default()
        };
        self.connections
            .insert((user_id, options.ssrc), conn.clone());
        conn
    }

    #[napi]
    pub fn set_transport_options(&mut self, options: CoreTransportOptions) {
        self.transport_opts.merge(options);
    }

    // Telemetry
    #[napi]
    pub fn get_codec_survey(&self, env: Env, callback: JsFunction) -> napi::Result<()> {
        let _ = callback.call(None, &[env.create_string("null\n")?]);
        Ok(())
    }
}
