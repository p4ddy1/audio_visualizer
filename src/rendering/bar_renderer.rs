use std::f64;

use crate::rendering::Renderer;

use super::sdl2::pixels::Color;
use super::sdl2::rect::Rect;
use super::sdl2::render::Canvas;
use super::sdl2::video::Window;

const BAR_HEIGHT: u32 = 6;
const BAR_COUNT: usize = 100;

pub struct BarRenderer {
    bar_buffer: Vec<Vec<f64>>,
}

impl BarRenderer {
    fn convert_data_to_bars(data: &[f64], bars: usize) -> Vec<f64> {
        let step = data.len() / bars;

        data.iter()
            .step_by(step + 1)
            .enumerate()
            .map(|(i, _)| data.iter().skip(i * step).take(step).sum::<f64>() / step as f64)
            .collect::<Vec<f64>>()
    }

    fn convert_dbfs_to_positive_value(dbfs: f64) -> i32 {
        if dbfs <= -100.0 {
            0
        } else {
            (100.0 + dbfs).round() as i32
        }
    }

    fn smooth_bar_values(&mut self, bars: &[f64]) {
        for (i, bar_value) in bars.iter().enumerate() {
            if self.bar_buffer[i].len() >= 4 {
                self.bar_buffer[i].remove(0);
            }
            self.bar_buffer[i].push(*bar_value);
        }
    }
}

impl Renderer for BarRenderer {
    fn new() -> Self {
        BarRenderer {
            bar_buffer: vec![Vec::new(); BAR_COUNT],
        }
    }

    fn render_data(&mut self, data: &[f64], canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let (width, height) = canvas.output_size().unwrap();

        let bars = BarRenderer::convert_data_to_bars(data, BAR_COUNT);
        self.smooth_bar_values(&bars);

        let bar_size = (width as f64 / bars.len() as f64).round() as u32;

        for (i, bar) in self.bar_buffer.iter().enumerate() {
            let avg_dbfs = bar.iter().sum::<f64>() / bar.len() as f64;
            let bar_value = BarRenderer::convert_dbfs_to_positive_value(avg_dbfs);

            //Draw one rectangle per value
            for box_count in 0..bar_value {
                let rect = Rect::new(
                    i as i32 * bar_size as i32,
                    height as i32 - (box_count * BAR_HEIGHT as i32),
                    bar_size,
                    BAR_HEIGHT,
                );

                canvas.set_draw_color(Color::RGB(
                    255 - (255 / self.bar_buffer.len() as u8) * i as u8,
                    (128 / self.bar_buffer.len() as u8) * (i / 2) as u8,
                    (255 / self.bar_buffer.len() as u8) * i as u8,
                ));
                canvas.draw_color();
                canvas.fill_rect(rect).unwrap();
            }
        }

        canvas.present();
    }
}
