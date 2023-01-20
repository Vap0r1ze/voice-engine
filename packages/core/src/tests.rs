use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Sample, SampleFormat, SampleRate,
};

struct AudioStream {
    stream: Option<cpal::Stream>,
    encoder: opus::Encoder,
}

fn create_audio_stream() -> AudioStream {
    let host = cpal::default_host();
    let input = host
        .default_input_device()
        .expect("Failed to get input device");
    let supported_input_range = input
        .supported_input_configs()
        .expect("Failed to get input config")
        .next()
        .expect("Failed to get next input config");
    let supported_sample_rates =
        supported_input_range.min_sample_rate().0..supported_input_range.max_sample_rate().0;
    println!("Supported sample rates: {:?}", supported_sample_rates);
    let sample_rate = [48000u32, 24000, 16000, 12000, 8000]
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

    let stream = match sample_format {
        SampleFormat::F32 => input.build_input_stream(
            &input_config,
            |pcm: &[f32], _: &_| read_sample::<f32>(pcm, &mut opus_encoder),
            err_fn,
        ),
        SampleFormat::I16 => input.build_input_stream(
            &input_config,
            |pcm: &[i16], _: &_| read_sample::<i16>(pcm, &mut opus_encoder),
            err_fn,
        ),
        SampleFormat::U16 => input.build_input_stream(
            &input_config,
            |pcm: &[u16], _: &_| read_sample::<u16>(pcm, &mut opus_encoder),
            err_fn,
        ),
    }
    .expect("Failed to build input stream");
    let audio_stream = AudioStream {
        stream,
        encoder: opus_encoder,
    };
    audio_stream
    // input
    //     .build_input_stream(
    //         &input_config.config(),
    //         |data: &[i16], _: &cpal::InputCallbackInfo| {
    //             println!("Data[{}]: {:?}", data.len(), data);
    //         },
    //         |err| {
    //             println!("Error: {:?}", err);
    //         },
    //     )
    //     .expect("Failed to build input stream");
    // stream.play().expect("Failed to play stream");
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // stream.pause().expect("Failed to pause stream");
}

fn read_sample<T: Sample>(pcm: &[T], encoder: &mut opus::Encoder) {
    println!(
        "Data[{}]: {:?}",
        pcm.len(),
        pcm.iter().map(|s| s.to_f32()).collect::<Vec<f32>>()
    );
    // for sample in data.iter() {
    //     sample.to_f32();
    // }
}
