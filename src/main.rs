use clap::{App, Arg};

use crate::rendering::bar_renderer::BarRenderer;
use crate::rendering::Renderer;
use crate::signal_processing::fft::FftProcessor;
use crate::source::pulse_source::PulseSource;
use crate::source::Source;
use crate::visualizer_core::VisualizerCore;

mod rendering;
mod signal_processing;
mod source;
mod visualizer_core;

fn main() {
    let matches = App::new("Audio Visualizer")
        .version("0.1")
        .author("Patric Kanngießer <mail@lpnw.de>")
        .about("Simple audio visualizer for educational purposes")
        .arg(Arg::with_name("source").help("PulseAudio source the visualizer should listen to"))
        .get_matches();

    let mut renderer = BarRenderer::new();
    let source = PulseSource::new(2048, matches.value_of("source"));
    let processor = FftProcessor::new(2048);
    let mut core = VisualizerCore::new(&mut renderer, source, processor);
    core.start();
}
