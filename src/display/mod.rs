use std::sync::mpsc;
use std::time::Duration;

use unicode_width::UnicodeWidthStr;
use anathema::{Color, Colors, Cursor, Input, Instruction, Line, Lines, Pos, ScrollBuffer, Size, Sub, Window};
use anyhow::Result;
use rand::prelude::*;
use rodio::OutputStream;

mod animation;
mod chat_display;
mod event_display;
mod fullscreen_display;
pub mod models;

use chat_display::ChatDisplay;
use event_display::EventDisplay;
use fullscreen_display::FullscreenDisplay;

pub type DisplayEventRx = mpsc::Receiver<models::DisplayMessage>;
pub type DisplayEventTx = mpsc::Sender<models::DisplayMessage>;

const EVENT_HEIGHT: i32 = 9;
const NAP_TIME: u64 = 30;

pub const GREY: Color = Color::Id(80);

pub fn channel() -> (DisplayEventTx, DisplayEventRx) {
    mpsc::channel()
}

// -----------------------------------------------------------------------------
//     - Setup colours -
// -----------------------------------------------------------------------------
fn setup_colors() {
    // Base colours
    for c in 1..8 {
        if let Err(e) = Colors::init_fg(Color::from(c)) {
            log::error!("Failed to init colour {}: {}", c, e);
        }
    }

    Colors::init_color(GREY.into(), 400, 400, 400).unwrap();
}

pub fn random_color() -> Color {
    const COLORS: [Color; 7] =
        [Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Magenta, Color::Cyan, Color::White];

    let mut rng = thread_rng();
    *COLORS.choose(&mut rng).expect("this really shouldn't fail")
}

// Get the sizes for the event view and the chat view.
// If there isn't enough space for the events, set the size to
// zero and don't draw it.
pub fn sizes(total: Size) -> (Size, Size) {
    let event_size = {
        match total.height > EVENT_HEIGHT {
            true => Size::new(total.width, EVENT_HEIGHT),
            false => Size::new(total.width, 0),
        }
    };
    let chat_size = match event_size.height {
        EVENT_HEIGHT => Size::new(total.width, total.height - EVENT_HEIGHT),
        _ => total,
    };

    (event_size, chat_size)
}

pub fn run(events: DisplayEventRx) -> Result<()> {
    let window = Window::main(true)?;
    window.no_delay(true)?;
    window.set_cursor_visibility(Cursor::Hide)?;
    setup_colors();

    let mut colors = Colors::new(9);

    let (_stream, sound_output_handle) = OutputStream::try_default()?;

    let (event_size, chat_size) = sizes(window.size());
    let event_win = window.new_window(Pos::new(0, 0), event_size)?;
    let chat_win = window.new_window(Pos::new(0, event_size.height), chat_size)?;
    let fullscreen_win = window.new_window(Pos::new(0, 0), window.size())?;

    let mut chat = ChatDisplay::new(chat_win);
    let mut event_disp = EventDisplay::new(event_win, sound_output_handle.clone(), None)?;
    let mut fullscreen = FullscreenDisplay::new(fullscreen_win, sound_output_handle);

    loop {
        // ---------------------------------------------------------------------
        //     - Incoming events -
        // ---------------------------------------------------------------------
        while let Ok(event) = events.try_recv() {
            chat.handle(&event);
            event_disp.handle(&event)?;
            fullscreen.handle(&event)?;
        }

        // ---------------------------------------------------------------------
        //     - Input handling -
        // ---------------------------------------------------------------------
        if let Some(key) = window.get_input() {
            chat.input(key)?;

            match key {
                Input::Character('c') => break Ok(()),
                Input::KeyResize => {
                    // ---------------------------------------------------------
                    //     - Resize all windows -
                    // ---------------------------------------------------------
                    let (event_size, chat_size) = sizes(window.size());

                    chat.move_win(Pos::new(0, event_size.height))?;
                    chat.resize(chat_size)?;
                    event_disp.resize(event_size)?;
                    fullscreen.resize(window.size())?;
                }
                _ => {}
            }
        }

        // ---------------------------------------------------------------------
        //     - Update and draw -
        // ---------------------------------------------------------------------
        while fullscreen.wants_update() && !event_disp.wants_update() {
            fullscreen.update()?;
            window.nap(Duration::from_millis(NAP_TIME))?;

            // If `fullscreen` is done drawing,
            // mark the chat and event display as dirty so they redraw.
            if !fullscreen.wants_update() {
                chat.touch();
                event_disp.touch();
            }
        }

        chat.update(&mut colors)?;
        event_disp.update()?;

        window.nap(Duration::from_millis(NAP_TIME))?;
    }
}

fn render_lines(lines: Lines<'_>, window: &Window<Sub>, offset: usize) -> Result<()> {
    let height = window.size().height as usize;
    let skip = (lines.len().max(height) - height).saturating_sub(offset).saturating_sub(1);

    for (i, line) in lines.iter().skip(skip).take(height).enumerate() {
        let mut ends_with_newline = false;

        for inst in line.instructions() {
            match inst {
                Instruction::String(mut s) => {
                    let cur = window.get_cursor();
                    let size = window.size();

                    // Note: this is a bit awkward, but ncurses can't print the last 
                    // char on the last line unless scrolling is disabled.
                    // This would cause it to scroll down one line and that would be naff!
                    // So instead we ignore the error that will arise when printing a
                    // character in the last column on the last line.
                    if s.width() as i32 + cur.x >= size.width && cur.y + 1 == size.height {
                        window.print(s)?;
                    } else {
                        window.print(s)?;
                    }
                }
                Instruction::Pad(n) => {
                    let width = window.size().width;
                    let mut cur = window.get_cursor();
                    let n = (*n as i32).min(width - (cur.x + 1));
                    cur.x += n;
                    window.move_cursor(cur)?;
                }
                Instruction::Color(c) => {
                    let pair = Colors::get_color_pair(*c);
                    window.set_color(pair)?;
                }
                Instruction::Style(s) => {
                    window.enable_style(*s)?;
                }
                Instruction::ResetColor => {
                    let pair = Colors::get_color_pair(7);
                    window.set_color(pair)?;
                }
                Instruction::ResetStyle => {
                    window.reset_style()?;
                }
            }
        }

        // TODO: add all these separate IFs back in
        // and make sure they behave

        if ends_with_newline {
            continue
        }

        // if i + 1 == height && line.width() >= window.size().width as usize {
        //     continue
        // }

        // if i + 1 == height {
        //     continue
        // }

        // if line.width() >= window.size().width as usize {
        //     continue
        // }

        // Don't add a newline char to the last line
        if i + 1 != height && line.width() < window.size().width as usize {
            if let Err(e) = window.add_char('\n') {
                log::error!("i: {} | len: {} | {}", i, height, e);
            }
        }
    }

    Ok(())
}
