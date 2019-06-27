use sdl2::event::Event;

use crate::rendering::Renderer;
use crate::signal_processing::SignalProcessor;
use crate::source::Source;

pub struct VisualizerCore<'a, R: Renderer, S: Source, P: SignalProcessor> {
    renderer: &'a mut R,
    source: S,
    processor: P,
}

impl<'a, R: Renderer, S: Source, P: SignalProcessor> VisualizerCore<'a, R, S, P> {
    pub fn new(renderer: &'a mut R, source: S, processor: P) -> Self {
        VisualizerCore {
            renderer,
            source,
            processor,
        }
    }

    pub fn start(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Audio Visualizer", 800, 600)
            .opengl()
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        'mainloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        break 'mainloop;
                    }
                    _ => {}
                }
            }

            let samples = self.source.read();
            let samples = self.processor.process(&samples);

            self.renderer.render_data(&samples, &mut canvas);
        }
    }
}
