use euclid::default::Vector2D;
use rand::prelude::*;
use tinybit::events::*;
use tinybit::render::RenderTarget;
use tinybit::{term_size, Camera, Pixel, Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport, Color};
use anyhow::Result;

#[derive(Debug)]
struct Char {
    c: char,
    current: Vector2D<f32>,
    dest: Vector2D<f32>,
    fg_color: Option<Color>,
}

impl Char {
    fn step(&mut self) {
        if (self.dest - self.current).length() < 0.7 {
            self.current = self.dest;
        } else {
            self.current += (self.dest - self.current).normalize();
        }
    }

    fn pixel(&self) -> Option<Pixel> {
        if self.current.x < 0.0 || self.current.y < 0.0 {
            None
        } else {
            let pos = self.current.cast::<u16>();
                let pix = Pixel::new(self.c, ScreenPos::new(pos.x, pos.y), self.fg_color, None);
            Some(pix)
        }
    }

    fn done(&self) -> bool {
        self.current == self.dest
    }
}

fn random_anim() -> Animation {
    const ANIMS: [Animation; 3] = [
        Animation::Rain,
        Animation::Scatter,
        Animation::WriteOut,
    ];

    let mut rng = thread_rng();
    *ANIMS.choose(&mut rng).unwrap()
}

#[derive(Debug, Copy, Clone)]
pub enum Animation {
    Rain,
    Scatter,
    WriteOut,
    NoAnimation,
}

fn get_chars(
    input: &str,
    animation: Animation,
    viewport: &mut Viewport,
) -> Vec<Char> {
    let mut rng = thread_rng();
    let lines = super::input::lines(input, viewport.size.width as usize);
    let max_width = lines
        .iter()
        .map(|l| l.chars().count())
        .max()
        .expect("lines should not be empty");
    let mut y = (viewport.size.height as usize / 2 - lines.len() / 2) as u16;
    let start_x = viewport.size.width / 2 - (max_width / 2) as u16;
    let mut x = start_x;

    let mut chars = Vec::new();

    for line in lines {
        for c in line.chars() {
            let current = match animation {
                Animation::Scatter => {
                    let x = *[1, viewport.size.width - 2]
                        .choose(&mut rng)
                        .expect("failed to get x");
                    let y = *[1, viewport.size.height - 2]
                        .choose(&mut rng)
                        .expect("failed to get x");
                    Vector2D::new(x as f32, y as f32)
                }
                Animation::Rain => {
                    let y = rng.gen_range(-40..0);
                    Vector2D::new(x as f32, y as f32)
                }
                Animation::WriteOut => {
                    let x = max_width as f32;
                    Vector2D::new(x as f32, y as f32)
                }
                Animation::NoAnimation => Vector2D::new(x as f32, y as f32),
            };

            let dest = Vector2D::new(x as f32, y as f32);

            chars.push(Char { c, dest, current, fg_color: Some(Color::Red) });

            x += 1;
        }
        x = start_x;
        y += 1;
    }

    chars
}

fn draw(chars: &[Char], viewport: &mut Viewport) {
    let pixels = chars.iter().filter_map(Char::pixel).collect::<Vec<_>>();
    viewport.draw_pixels(&pixels);
}

fn animate(text: &str, animation: Animation) -> Result<()> {
    let (width, height) = term_size()?;

    let mut viewport = Viewport::new(ScreenPos::new(0, 0), ScreenSize::new(width, height));

    let stdout = StdoutTarget::new()?;
    let mut renderer = Renderer::new(stdout);

    let mut chars = get_chars(text, animation, &mut viewport);

    Ok(())
}

enum Stage {
    Animating,
    Displaying(usize),
    Done,
}

pub struct Anim {
    chars: Vec<Char>,
    stage: Stage,
}

impl Anim {
    pub fn new(input: String, animation: Animation, viewport: &mut Viewport) -> Self {
        let chars = get_chars(&input, animation, viewport);
        Self {
            chars,
            stage: Stage::Animating,
        }
    }

    pub fn update(&mut self, renderer: &mut Renderer<StdoutTarget>, viewport: &mut Viewport) -> bool {
        self.chars.iter_mut().for_each(Char::step);
        draw(&self.chars, viewport);
        renderer.render(viewport);
        viewport.swap_buffers();

        match self.stage {
            Stage::Animating if self.chars.iter().filter(|c| !c.done()).count() == 0 => {
                self.stage = Stage::Displaying(1000);
                false
            }
            Stage::Displaying(ref mut val) => {
                *val -= 20;
                if *val == 0 {
                    self.stage = Stage::Done;
                }
                false
            }
            Stage::Animating => false,
            Stage::Done => true,
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use super::input::lines;

//     #[test]
//     fn lines_split() {
//         let expected = vec!["one".to_string(), "two".to_string()];
//         let actual = lines("one\ntwo", usize::MAX);
//         assert_eq!(expected, actual);
//     }

//    #[test]
//     // Lines that are longer than the screen width
//     // are split on white space
//     fn long_lines_split() {
//         let input = "long line";
//         let expected = vec!["long", "line"];
//         let screen_width = "long li".len();
//         let actual = lines(input, screen_width);
//         assert_eq!(expected, actual);
//     }

//     #[test]
//     // Line is longer than the screen width,
//     // and has no spacing
//     fn long_line_split() {
//         let input = "1234567890ABCDE";
//         let expected = vec!["12345", "67890", "ABCDE"];
//         let screen_width = 5;
//         let actual = lines(input, screen_width);
//         assert_eq!(expected, actual);
//     }

//     #[test]
//     // Split on other
//     fn split_on_tab() {
//         let input = "long\tline";
//         let expected = vec!["long", "line"];
//         let screen_width = "long li".len();
//         let actual = lines(input, screen_width);
//         assert_eq!(expected, actual);
//     }
// }

