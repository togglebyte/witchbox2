use anathema::{split, Pos, Size};
use rand::prelude::*;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Copy, Clone)]
pub struct Char {
    pub current_pos: Pos,
    dest: Pos,
    pub c: char,
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
struct CharAnim {
    chars: Vec<Char>,
    is_done: bool,
    life: usize,
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
                chars.push(Char {
                    dest: Pos::new(x as i32 + dest_x, y as i32 + dest_y),
                    current_pos: Pos::new(rng.gen_range(0..size.width), 1),
                    c,
                });
            }
        }

        Self { chars, is_done: false, life: words.len() * 3 }
    }

    fn update(&mut self) -> Vec<Char> {
        for c in &mut self.chars {
            if c.current_pos != c.dest {
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
            }
        }

        let mut is_done = true;

        // super dodge
        for c in &self.chars {
            if c.dest != c.current_pos {
                is_done = false;
            }
        }

        if is_done {
            self.life -= 1;
        }

        if !self.is_done && is_done && self.life == 0 {
            self.is_done = true;
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
