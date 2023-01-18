#![deny(clippy::all)]

use std::collections::HashMap;

use callbacks::{CallbackRef, CoreCallbackStore};
use connection::{ConnectionOptions, VoiceConnection};
use napi::{Env, JsFunction, JsNumber};
use napi_derive::napi;

mod callbacks;
mod connection;
mod crypt;

#[napi]
#[derive(Default)]
pub struct VoiceCore {
    callbacks: CoreCallbackStore,
    connections: HashMap<(u64, u32), VoiceConnection>,
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
    pub fn _create_voice_connection(
        &mut self,
        user_id: String,
        options: ConnectionOptions,
    ) -> VoiceConnection {
        let conn = VoiceConnection {
            user_id: user_id.to_string(),
            options: ConnectionOptions {
                address: options.address,
                port: options.port,
                ssrc: options.ssrc,
                modes: options.modes,
                stream_params: options.stream_params,
                stream_user_id: options.stream_user_id,
            },
        };
        let id_num = user_id.parse::<u64>().unwrap();
        self.connections
            .insert((id_num, options.ssrc), conn.clone());
        conn
    }
}

#[napi]
fn _start() -> VoiceCore {
    Default::default()
}
