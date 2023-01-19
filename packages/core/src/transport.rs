use std::collections::HashMap;

use core_macros::str_enum;
use napi::{bindgen_prelude::ToNapiValue, Either};
use napi_derive::napi;

use crate::{connection::StreamParameters, crypt::CipherMode};

#[macro_export]
macro_rules! patch_options {
    ($resource:expr, $payload:expr, $( $key:ident ),+) => {
        {
            $( $resource.$key = $payload.$key.or($resource.$key); )*
        }
    };
}

macro_rules! partial {
    ($( #[$attr:meta] )* pub struct $name:ident {
        $( pub $field_name:ident: $field_type:ty, )*
    }) => {
        $( #[$attr] )*
        pub struct $name {
            $(pub $field_name: Option<$field_type>,)*
        }

        impl $name {
            pub fn merge(&mut self, other: $name) {
                $( if other.$field_name.is_some() { self.$field_name = other.$field_name; } )*
            }
        }
    }
}

#[napi]
pub enum DegradationPreference {
    MaintainResolution,
    MaintainFramerate,
    Balanced,
    Disabled,
}

#[napi]
pub enum AudioInputMode {
    Activity,
    PushToTalk,
}
#[napi(object)]
#[derive(Clone)]
pub struct VADOptions {
    pub vad_auto_threshold: f64,
    pub vad_threshold: f64,
    pub vad_leading: f64,  // TODO: are these ints?
    pub vad_trailing: f64, // TODO: are these ints?
    pub vad_use_krisp: bool,
}
#[napi(object)]
#[derive(Clone)]
pub struct PushToTalkOptions {
    pub ptt_release_delay: i32,
}

partial! {
    #[napi(object)]
    #[derive(Default)]
    pub struct CoreTransportOptions {
        pub automatic_gain_control: bool,
        pub built_in_echo_cancellation: bool,
        pub echo_cancellation: bool,
        pub noise_cancellation: bool,
        pub noise_suppression: bool,

        pub h264_enabled: bool,
        pub av1_enabled: bool,
        pub ducking: bool,
        pub idle_jitter_buffer_flush: bool,
    }
}

partial! {
    #[napi(object)]
    #[derive(Clone, Default)]
    pub struct ConnectionTransportOptions {
        pub hardware_h264: bool,

        pub input_mode: AudioInputMode,
        pub input_mode_options: Either<VADOptions, PushToTalkOptions>,

        pub encoding_voice_bit_rate: i32,
        pub attenuate_while_speaking_others: bool,
        pub attenuate_while_speaking_self: bool,
        pub attenuation: bool,
        pub attenuation_factor: f64,
        pub priority_speaker_ducking: f64,

        pub call_bit_rate: i32,
        pub call_min_bit_rate: i32,
        pub call_max_bit_rate: i32,

        pub encoding_video_bit_rate: i32,
        pub encoding_video_frame_rate: i32,
        pub encoding_video_max_bit_rate: i32,
        pub encoding_video_min_bit_rate: i32,
        pub encoding_video_height: i32,
        pub encoding_video_width: i32,
        pub remote_sink_wants_max_framerate: i32,
        pub remote_sink_wants_pixel_count: i32,
        pub stream_parameters: Vec<StreamParameters>,
        pub minimum_jitter_buffer_level: i32,
        pub encoding_video_degradation_preference: DegradationPreference,

        pub video_encoder: VideoCodec,
        pub video_decoders: Vec<VideoCodec>,
        pub audio_encoder: AudioEncoder,
        pub audio_decoders: Vec<AudioCodec>,
        pub experimental_encoders: bool,

        pub qos: bool,
        pub reconnect_interval: i32,

        pub self_mute: bool,
        pub self_deafen: bool,

        pub fec: bool,

        pub packet_loss_rate: i32,
        pub postpone_decode_level: i32,

        pub encryption_settings: EncryptionSettings,
        pub user_channel_ids: UserChannelIds,
    }
}

#[str_enum]
pub enum VideoCodecName {
    H264 = "H264",
    VP8 = "VP8",
    VP9 = "VP9",
}
#[napi(object)]
#[derive(Clone)]
pub struct VideoCodec {
    pub name: VideoCodecName,
    pub r#type: u32,
    pub rtx_type: u32,
    pub params: Option<HashMap<String, String>>,
}

#[str_enum]
pub enum AudioCodecName {
    Opus = "opus",
}
#[napi(object)]
#[derive(Clone)]
pub struct AudioEncoder {
    pub name: AudioCodecName,
    pub channels: u32,
    pub freq: i32,
    pub r#type: i32,
    pub params: Option<HashMap<String, String>>,
}
// TODO: figure out how to reuse these props
#[napi(object)]
#[derive(Clone)]
pub struct AudioCodec {
    pub name: AudioCodecName,
    pub channels: u32,
    pub freq: i32,
    pub r#type: i32,
    pub params: Option<HashMap<String, String>>,
    pub rate: u32,
    pub pacsize: u32,
}

#[napi(object)]
#[derive(Clone)]
pub struct EncryptionSettings {
    pub mode: CipherMode,
    pub secret_key: Vec<u32>,
}

#[napi(object)]
#[derive(Clone)]
pub struct UserChannelIds {
    pub user_id: String,
    pub channel_id: String,
    pub guild_id: Option<String>,
}
