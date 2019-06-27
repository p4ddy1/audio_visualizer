extern crate rustfft;

use rustfft::algorithm::Radix4;
use rustfft::FFT;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use crate::signal_processing::SignalProcessor;

pub struct FftProcessor {
    radix: Radix4<f64>,
    window: Vec<f64>,
    buffer_size: usize,
}

impl FftProcessor {
    pub fn new(buffer_size: usize) -> Self {
        let radix = Radix4::new(buffer_size, false);
        let window = apodize::hanning_iter(buffer_size).collect::<Vec<f64>>();

        FftProcessor {
            radix,
            window,
            buffer_size,
        }
    }

    fn normalize_magnitude(&self, value: &Complex<f64>) -> f64 {
        f64::sqrt(value.re * value.re + value.im * value.im) / (self.buffer_size as f64 / 2.0)
    }
}

impl SignalProcessor for FftProcessor {
    fn process(&self, samples: &[i16]) -> Vec<f64> {
        let mut fft_input = Vec::new();
        let mut fft_output = vec![Complex::zero(); self.buffer_size];
        let mut output: Vec<f64> = Vec::new();

        for (i, sample) in samples.iter().enumerate() {
            fft_input.push(Complex::from(*sample as f64 * self.window[i]));
        }

        self.radix.process(&mut fft_input, &mut fft_output);

        //Skip first entry because its DC Offset
        for number in fft_output.iter().take(fft_output.len() / 2).skip(1) {
            let real = self.normalize_magnitude(number);

            //Only calculate dBFs for positive bins
            let db = if real > 0.0 {
                self.calculate_db_fs(real)
            } else {
                FftProcessor::MIN_DB
            };

            output.push(db);
        }

        output
    }
}
