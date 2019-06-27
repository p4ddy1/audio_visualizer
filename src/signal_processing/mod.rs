use std::i16;

pub mod fft;

pub trait SignalProcessor {
    const MIN_DB: f64 = -100.0;

    fn process(&self, samples: &[i16]) -> Vec<f64>;

    fn calculate_db_fs(&self, value: f64) -> f64 {
        20.0 * (f64::log(2.0 * value / i16::MAX as f64, 10.0))
    }
}
