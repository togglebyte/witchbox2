use tinybit::{Renderer, StdoutTarget};

pub trait View {
    fn draw(&mut self, renderer: &mut Renderer<StdoutTarget>);
    fn resize(&mut self, width: u16, height: u16);
}
