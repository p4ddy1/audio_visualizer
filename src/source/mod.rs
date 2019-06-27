pub mod pulse_source;

pub trait Source {
    fn new(buffer_size: usize, device: Option<&str>) -> Self;
    fn read(&mut self) -> Vec<i16>;
}
