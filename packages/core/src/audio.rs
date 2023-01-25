use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

use napi_derive::napi;

static OPUS_SAMPLE_RATES: [u32; 5] = [48000, 24000, 16000, 12000, 8000];

pub struct AudioManager {
    host: cpal::Host,
    frame_rx: mpsc::Receiver<AudioFrame>,
    frame_tx: mpsc::Sender<AudioFrame>,
    stream: Option<cpal::Stream>,
}
pub struct FrameRequest {
    pub from_rate: u32,
    pub to_rate: u32,
    pub from_channels: usize,
    pub to_channels: usize,
}
type AudioFrame = Vec<f32>;

pub enum AudioDeviceType {
    Input,
    Output,
}

pub struct StreamRequest {
    pub sample_rate: u32,
    pub channels: u16,
}

impl AudioManager {
    pub fn new() -> Self {
        let (frame_tx, frame_rx) = mpsc::channel();
        Self {
            host: cpal::default_host(),
            frame_tx,
            frame_rx,
            stream: None,
        }
    }

    pub fn get_input_devices(&self) -> Result<Vec<cpal::Device>, cpal::DevicesError> {
        Ok(self.host.input_devices()?.collect())
    }
    pub fn get_default_input_device(&self) -> Option<cpal::Device> {
        self.host.default_input_device()
    }
    pub fn get_output_devices(&self) -> Result<Vec<cpal::Device>, cpal::DevicesError> {
        Ok(self.host.output_devices()?.collect())
    }
    pub fn get_default_output_device(&self) -> Option<cpal::Device> {
        self.host.default_output_device()
    }

    pub fn start_input_stream(&mut self, input: &cpal::Device, stream_req: &StreamRequest) {
        let supported_input = input
            .supported_input_configs()
            .expect("Failed to get system input config")
            .next()
            .expect("Failed to get system input config")
            .with_max_sample_rate();

        let stream_config = supported_input.config();

        print!("Supported input config: {:?}", supported_input);
        print!("Input config: {:?}", stream_config);

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let sample_format = supported_input.sample_format();

        let frame_req = FrameRequest {
            from_rate: stream_config.sample_rate.0,
            to_rate: stream_req.sample_rate,
            from_channels: stream_config.channels.into(),
            to_channels: stream_req.channels.into(),
        };

        let tx = self.frame_tx.clone();

        macro_rules! build_input_stream {
            ($type:ty) => {
                input.build_input_stream(
                    &stream_config,
                    move |pcm: &[$type], _: &_| send_input_frame::<$type>(pcm, &frame_req, &tx),
                    err_fn,
                )
            };
        }

        let stream = match sample_format {
            cpal::SampleFormat::I16 => build_input_stream! {i16},
            cpal::SampleFormat::U16 => build_input_stream! {u16},
            cpal::SampleFormat::F32 => build_input_stream! {f32},
        }
        .expect("Failed to build input stream");
        stream.play().expect("Failed to play stream");

        std::thread::sleep(std::time::Duration::from_secs(1));
        drop(stream);
    }
}

fn send_input_frame<T: cpal::Sample>(
    pcm: &[T],
    frame_req: &FrameRequest,
    tx: &mpsc::Sender<AudioFrame>,
) {
    let input = pcm.iter().map(|s| s.to_f32()).collect::<Vec<f32>>();

    // TODO: Stereo <-> Mono conversion

    let resampled = samplerate::convert(
        frame_req.from_rate,
        frame_req.to_rate,
        frame_req.from_channels,
        samplerate::ConverterType::SincBestQuality,
        &input,
    )
    .expect("Failed to resample");
    tx.send(resampled).expect("Failed to send audio frame");
}
