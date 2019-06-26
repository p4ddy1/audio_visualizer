mod drawing;


pub trait Drawable {
    fn render_data(&mut self, data: &[i32]);
}