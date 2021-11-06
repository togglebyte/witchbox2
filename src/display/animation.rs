use std::fs::read_to_string;
use std::path::Path;
use std::time::{Duration, Instant};

use rand::prelude::*;
use anathema::{split, Color, Instruction, Lines, Line, Pos, Size};
use unicode_width::UnicodeWidthStr;

use crate::display::random_color;


#[derive(Debug, Copy, Clone)]
pub struct Char {
    pub c: char,
    pub current_pos: Pos,
    pub color: Color,
    start: Pos,
    dest: Pos,
}

// -----------------------------------------------------------------------------
//     - Character movement anim -
// -----------------------------------------------------------------------------
enum AnimState {
    In,
    Wait(Instant),
    Out,
    Done,
}

pub struct CharAnim {
    chars: Vec<Char>,
    pub is_done: bool,
    ttl: Duration,
    state: AnimState,
}

pub enum Animation {
    Scatter,
    HorzSlide,
    VertSlide,
}

fn animation_chars(lines: Vec<&str>, size: Size, animation: Animation) -> Vec<Char> {
    let mut chars = vec![];
    let dest_y = size.height / 2 - lines.len() as i32 / 2;
    let longest_line = lines.iter().map(|l| l.width()).max().unwrap_or(1) as i32;
    let dest_x = size.width / 2 - longest_line / 2;

    let line_count = lines.len() as i32;
    let color = random_color();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().filter(|c| *c != '\n').enumerate() {
            let x = x as i32;
            let y = y as i32;
            let mut rng = thread_rng();

            let start = match animation {
                Animation::Scatter => Pos::new(rng.gen_range(0..size.width), 1),
                Animation::HorzSlide => Pos::new(x + size.width, y + dest_y),
                Animation::VertSlide => Pos::new(x + dest_x, y + size.height),
            };

            chars.push(Char {
                dest: Pos::new(x as i32 + dest_x, y as i32 + dest_y),
                current_pos: start,
                start,
                c,
                color,
            });
        }
    }

    chars
}

impl CharAnim {
    pub fn new(words: &str, size: Size, animation: Animation) -> Self {
        let lines = split(words, size.width as usize, 0, true).collect::<Vec<_>>();

        let mut chars = animation_chars(lines, size, animation);

        let wpm = words.split_whitespace().count() as f32 * 0.25 + 3.0;

        let ttl = Duration::from_secs(wpm as u64);
        Self { chars, is_done: false, ttl, state: AnimState::In }
    }

    pub fn update(&mut self) -> Vec<Char> {
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
                    remove.into_iter().rev().for_each(|i| {
                        self.chars.remove(i);
                    });
                }

                if is_done {
                    match self.state {
                        AnimState::In => {
                            self.state = AnimState::Wait(Instant::now());
                        }
                        AnimState::Out => self.state = AnimState::Done,
                        _ => unreachable!(),
                    }
                }
            }
            AnimState::Wait(instant) => {
                if instant.elapsed() > self.ttl {
                    self.state = AnimState::Out;
                    self.chars.iter_mut().for_each(|c| c.dest = c.start);
                }
            }
            AnimState::Done => {
                self.is_done = true;
            }
        }

        self.chars.clone()
    }
}

// -----------------------------------------------------------------------------
//     - Frame animation -
// -----------------------------------------------------------------------------
enum FrameAnimState {
    NotStarted,
    Playing(Instant),
}

pub struct FrameAnim {
    frame_padding: usize,
    frames: Vec<Frame>,
    current_frame: usize,
    screen_width: usize,
    ttl: Duration,
    state: FrameAnimState,
    pub is_done: bool,
}

impl FrameAnim {
    pub fn new(path: impl AsRef<Path>, screen_width: usize) -> Self {
        let raw = read_to_string(path).unwrap();
        let mut lines = raw.lines();

        let lines_per_frame = lines.next().unwrap().parse::<usize>().unwrap();
        let frame_width = lines.next().unwrap().parse::<usize>().unwrap();

        let frame_padding = if screen_width > frame_width {
            // Pad
            (screen_width - frame_width) / 2
        } else {
            0
        };

        // A lot of allocations here... ¬_¬
        let frames = lines
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .chunks(lines_per_frame)
            .map(|l| Frame { lines: l.into_iter().map(|s| s.to_string()).collect() })
            .collect();

        Self {
            frame_padding,
            frames,
            current_frame: 0,
            screen_width,
            ttl: Duration::from_secs(4),
            is_done: false,
            state: FrameAnimState::NotStarted,
        }
    }

    pub fn update(&mut self) -> Lines {
        let started = match self.state {
            FrameAnimState::NotStarted => {
                let started = Instant::now();
                self.state = FrameAnimState::Playing(started);
                started
            }
            FrameAnimState::Playing(started) => started,
        };

        let mut lines = Lines::new(self.screen_width);

        if started.elapsed() > self.ttl {
            self.is_done = true;
            return lines;
        }

        for frame_line in &self.frames[self.current_frame].lines {
            lines.pad(1); // pad one to avoid drawing over the border
            // let mut line = Line::new();

            // line.pad(1); // pad one to avoid drawing over the border
            if self.frame_padding > 0 {
                lines.pad(self.frame_padding);
            }

            match frame_line.width() > self.screen_width {
                true => lines.push_str(&frame_line[..self.screen_width], true),
                false => lines.push_str(frame_line, true),
            }
        }

        self.current_frame += 1;
        if self.current_frame == self.frames.len() {
            self.current_frame = 0;
        }

        lines
    }
}

struct Frame {
    lines: Vec<String>,
}
