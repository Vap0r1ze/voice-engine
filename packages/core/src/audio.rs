use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, SampleFormat, SampleRate,
};

static OPUS_SAMPLE_RATES: [u32; 5] = [48000, 24000, 16000, 12000, 8000];

pub struct AudioManager {
    host: cpal::Host,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            host: cpal::default_host(),
        }
    }

    pub fn get_devices(&self) -> Result<Vec<cpal::Device>, cpal::DevicesError> {
        Ok(self.host.devices()?.collect())
    }

    pub fn create_input_stream(&self, input: &cpal::Device) {
        let supported_input_range = input
            .supported_input_configs()
            .expect("Failed to get input config")
            .next()
            .expect("Failed to get next input config");

        let supported_sample_rates =
            supported_input_range.min_sample_rate().0..supported_input_range.max_sample_rate().0;

        println!("Supported sample rates: {:?}", supported_sample_rates);

        let sample_rate = OPUS_SAMPLE_RATES
            .iter()
            .find(|&&sample_rate| supported_sample_rates.contains(&sample_rate))
            .expect("Failed to find supported sample rate");

        let supported_input = supported_input_range.with_sample_rate(SampleRate(*sample_rate));
        let input_config = supported_input.config();

        print!("Supported input config: {:?}", supported_input);
        print!("Input config: {:?}", input_config);

        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
        let sample_format = supported_input.sample_format();

        let mut opus_encoder = opus::Encoder::new(
            input_config.sample_rate.0,
            match input_config.channels {
                1 => opus::Channels::Mono,
                2 => opus::Channels::Stereo,
                _ => panic!("Unsupported number of channels"),
            },
            opus::Application::Voip,
        )
        .expect("Failed to create opus encoder");

        let writer = Arc::new(Mutex::new(opus_encoder));
        let writer_2 = writer.clone();

        macro_rules! build_input_stream {
            ($type:ty) => {
                input.build_input_stream(
                    &input_config,
                    |pcm: &[$type], _: &_| read_sample::<$type>(pcm, &writer_2),
                    err_fn,
                )
            };
        }

        let stream = match sample_format {
            SampleFormat::I16 => build_input_stream! {i16},
            SampleFormat::U16 => build_input_stream! {u16},
            SampleFormat::F32 => build_input_stream! {f32},
        }
        .expect("Failed to build input stream");
        stream.play().expect("Failed to play stream");
        std::thread::sleep(std::time::Duration::from_secs(1));
        drop(stream);
        writer.lock().unwrap().deref();
    }
}

fn read_sample<T: Sample>(pcm: &[T], encoder: &Arc<Mutex<opus::Encoder>>) {
    println!(
        "Data[{}]: {:?}",
        pcm.len(),
        pcm.iter().map(|s| s.to_f32()).collect::<Vec<f32>>()
    );
}
