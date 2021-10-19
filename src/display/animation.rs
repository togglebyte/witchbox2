use anathema::{split, Color, Pos, Size};
use rand::prelude::*;
use unicode_width::UnicodeWidthStr;

fn random_color() -> Color {
    const COLORS: [Color; 7] = [Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan, Color::White];

    let mut rng = thread_rng();
    *COLORS.choose(&mut rng).unwrap()
}

#[derive(Debug, Copy, Clone)]
pub struct Char {
    pub c: char,
    pub current_pos: Pos,
    pub color: Color,
    start: Pos,
    dest: Pos,
}

enum AnimType {
    Char(CharAnim),
    Frame(FrameAnim),
}

// -----------------------------------------------------------------------------
//     - Animation -
// -----------------------------------------------------------------------------
pub struct Animation {
    anim_type: AnimType,
    is_done: bool,
}

impl Animation {
    pub fn char_anim(words: &str, size: Size) -> Self {
        let anim = CharAnim::new(words, size);

        Self { anim_type: AnimType::Char(anim), is_done: false }
    }

    pub fn update(&mut self) -> Vec<Char> {
        match self.anim_type {
            AnimType::Char(ref mut anim) => {
                let chars = anim.update();

                if anim.is_done {
                    self.is_done = true;
                }

                chars
            }
            AnimType::Frame(ref mut anim) => {
                let chars = anim.update();
                chars
            }
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_done
    }
}

// -----------------------------------------------------------------------------
//     - Character movement anim -
// -----------------------------------------------------------------------------
enum AnimState {
    In,
    Wait,
    Out,
    Done,
}

struct CharAnim {
    chars: Vec<Char>,
    is_done: bool,
    life: usize,
    state: AnimState,
}

impl CharAnim {
    fn new(words: &str, size: Size) -> Self {
        let lines = split(words, size.width as usize, 0);
        let mut chars = Vec::new();

        let dest_y = size.height / 2 - lines.len() as i32 / 2;
        let dest_x = size.width / 2 - lines.iter().map(|l| l.width()).max().unwrap_or(1) as i32;
        for (y, line) in lines.into_iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let mut rng = thread_rng();
                let start = Pos::new(rng.gen_range(0..size.width), 1);
                chars.push(Char {
                    dest: Pos::new(x as i32 + dest_x, y as i32 + dest_y),
                    current_pos: start,
                    start,
                    c,
                    color: random_color(),
                });
            }
        }

        Self { chars, is_done: false, life: words.len() * 3, state: AnimState::In }
    }

    fn update(&mut self) -> Vec<Char> {
        match self.state {
            AnimState::In | AnimState::Out => {
                let mut is_done = true;

                let mut remove = Vec::new();
                for (index, c) in &mut self.chars.iter_mut().enumerate() {
                    if c.current_pos != c.dest {
                        is_done = false;
                        let v = (c.dest - c.current_pos).abs();

                        if v.x > v.y {
                            if c.current_pos.x > c.dest.x {
                                c.current_pos.x -= 1;
                            }
                            if c.current_pos.x < c.dest.x {
                                c.current_pos.x += 1;
                            }
                        } else {
                            if c.current_pos.y > c.dest.y {
                                c.current_pos.y -= 1;
                            }
                            if c.current_pos.y < c.dest.y {
                                c.current_pos.y += 1;
                            }
                        }

                        if c.current_pos == c.dest {
                            remove.push(index);
                        }
                    }
                }

                // Remove every char that has reached its destination,
                // when animating out
                if let AnimState::Out = self.state {
                    remove.into_iter().rev().for_each(|i| { self.chars.remove(i); });
                }

                if is_done {
                    match self.state {
                        AnimState::In => self.state = AnimState::Wait,
                        AnimState::Out => self.state = AnimState::Done,
                        _ => unreachable!(),
                    }
                }
            }
            AnimState::Wait => {
                self.life -= 1;
                if self.life == 0 {
                    self.state = AnimState::Out;
                    self.chars.iter_mut().for_each(|c| c.dest = c.start);
                }
            }
            AnimState::Done => self.is_done = true,
        }

        self.chars.clone()
    }
}

// -----------------------------------------------------------------------------
//     - Frame animation -
// -----------------------------------------------------------------------------
struct FrameAnim {
    frames: Vec<Frame>,
}

impl FrameAnim {
    fn update(&mut self) -> Vec<Char> {
        unimplemented!()
    }
}

struct Frame {}
