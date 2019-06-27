extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;

use crate::source::Source;

use self::psimple::Simple;
use self::pulse::sample::Spec;
use self::pulse::stream::Direction;

pub struct PulseSource {
    pulse: Simple,
    pulse_buffer: Vec<u8>,
}

impl Source for PulseSource {
    fn new(buffer_size: usize, device: Option<&str>) -> Self {
        let spec = Spec {
            format: pulse::sample::Format::S16le,
            rate: 44100,
            channels: 1,
        };

        let pulse = Simple::new(
            None,
            "Audio Visualizer",
            Direction::Record,
            device,
            "Input",
            &spec,
            None,
            None,
        )
        .unwrap();

        //Create a buffer twice as big as the desired size because pulse gives us unsigned 8bit chunks
        let pulse_buffer: Vec<u8> = vec![0; buffer_size * 2];

        PulseSource {
            pulse,
            pulse_buffer,
        }
    }

    fn read(&mut self) -> Vec<i16> {
        self.pulse.read(&mut self.pulse_buffer).unwrap();

        //Convert unsigned 8bit sample chunks from pulse to a vector of 16 bit samples
        self.pulse_buffer
            .iter()
            .enumerate()
            .step_by(2)
            .map(|(i, sample)| (self.pulse_buffer[i + 1] as i16) << 8 | *sample as i16)
            .collect::<Vec<i16>>()
    }
}
