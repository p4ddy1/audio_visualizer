use crate::rendering::bar_renderer::BarRenderer;
use crate::source::pulse_source::PulseSource;
use crate::rendering::Renderer;
use std::thread::sleep;
use std::time::Duration;
use crate::source::Source;
use hound::WavWriter;

mod rendering;
mod source;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("p4ddys Fancy Visualizer", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut renderer = BarRenderer::new(&mut canvas);

    let mut source = PulseSource::new(2048);

    let spec2 = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };


    for i in 0..10000000 {
        let data = source.read();

        for sample in data {
            wav_writer.write_sample(*sample);
        }
    }
}

