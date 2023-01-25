#![deny(clippy::all)]

use audio::{AudioDeviceType, AudioManager};
use callbacks::{CallbackRef, CoreCallbackStore};
use connection::{ConnectionOptions, VoiceConnectionHandle};
use cpal::traits::DeviceTrait;
use napi::{sys::PropertyAttributes::default, Env, JsFunction, JsNumber, JsObject};
use napi_derive::napi;
use transport::CoreTransportOptions;

mod audio;
mod callbacks;
mod connection;
mod crypt;
mod macros;
#[cfg(test)]
mod tests;
mod transport;

#[napi]
pub struct VoiceCore {
    callbacks: CoreCallbackStore,
    transport_opts: CoreTransportOptions,
    audio_manager: AudioManager,
}

#[napi]
fn _start() -> VoiceCore {
    VoiceCore {
        callbacks: Default::default(),
        transport_opts: Default::default(),
        audio_manager: AudioManager::new(),
    }
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
    ) -> VoiceConnectionHandle {
        VoiceConnectionHandle::new(user_id, options)
    }

    #[napi]
    pub fn set_transport_options(&mut self, options: CoreTransportOptions) {
        self.transport_opts.merge(options);
    }

    #[napi]
    pub fn get_input_devices(&self, env: Env, callback: JsFunction) -> napi::Result<()> {
        callback.call(None, &[self.get_js_devices(&env, AudioDeviceType::Input)?])?;
        Ok(())
    }

    #[napi]
    pub fn get_output_devices(&self, env: Env, callback: JsFunction) -> napi::Result<()> {
        callback.call(None, &[self.get_js_devices(&env, AudioDeviceType::Output)?])?;
        Ok(())
    }

    fn get_js_devices(&self, env: &Env, device_type: AudioDeviceType) -> napi::Result<JsObject> {
        let type_name = match device_type {
            AudioDeviceType::Input => "input",
            AudioDeviceType::Output => "output",
        };
        let devices = match device_type {
            AudioDeviceType::Input => self.audio_manager.get_input_devices(),
            AudioDeviceType::Output => self.audio_manager.get_output_devices(),
        }
        .map_err(|err| {
            napi::Error::from_reason(format!("Failed to get {type_name} devices: {err}"))
        })?;

        if devices.is_empty() {
            return Err(napi::Error::from_reason(format!(
                "No audio {type_name} devices found"
            )));
        };

        let mut js_devices = env.create_array_with_length(devices.len() + 1)?;

        let system_default_device = match device_type {
            AudioDeviceType::Input => self.audio_manager.get_default_input_device(),
            AudioDeviceType::Output => self.audio_manager.get_default_output_device(),
        };

        let default_device = system_default_device.as_ref().unwrap_or(&devices[0]);

        for (i, device) in std::iter::once(default_device)
            .chain(devices.iter())
            .enumerate()
        {
            let mut js_device = env.create_object()?;
            let device_index = env.create_int32(i as i32 - 1)?;
            let device_name = device.name().map_err(|err| {
                napi::Error::from_reason(format!("Failed to get device name: {}", err))
            })?;
            js_device.set_named_property("name", env.create_string(&device_name)?)?;
            js_device.set_named_property("index", device_index)?;
            js_devices.set_element(i as u32, js_device)?;
        }
        Ok(js_devices)
    }

    // Telemetry
    #[napi]
    pub fn get_codec_survey(&self, env: Env, callback: JsFunction) -> napi::Result<()> {
        let _ = callback.call(None, &[env.create_string("null\n")?]);
        Ok(())
    }
}
