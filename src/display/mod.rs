use std::sync::mpsc;
use std::time::Duration;

use anathema::{Colors, Cursor, Input, Instruction, Pos, ScrollBuffer, Window, Line};
use anyhow::Result;

mod chat;
pub mod models;
use chat::Chat;
use models::{ChatMessage, Display};

pub type DisplayEventRx = mpsc::Receiver<models::Display>;
pub type DisplayEventTx = mpsc::Sender<models::Display>;

pub fn channel() -> (DisplayEventTx, DisplayEventRx) {
    mpsc::channel()
}

pub fn run(events: DisplayEventRx) -> Result<()> {
    // * Create window
    // * Setup colours
    // * Create an event buffer (store N events)

    let window = Window::main(true)?;
    window.set_cursor_visibility(Cursor::Hide)?;
    window.no_delay(true);

    let chat_win = window.new_window(Pos::new(0, 0), window.size())?;
    let mut chat = Chat::new(chat_win);

    let mut event_buffer = Vec::with_capacity(1024);

    loop {
        // -----------------------------------------------------------------------------
        //     - Incoming events -
        // -----------------------------------------------------------------------------
        while let Ok(event) = events.try_recv() {
            // Decide which `Display` gets the event
            match &event {
                Display::Chat(message) => chat.handle_message(message),
                Display::ClearChat => chat.clear_chat(),
            }

            // event_to_display(&event);
            event_buffer.push(event);
        }

        // -----------------------------------------------------------------------------
        //     - Input handling -
        // -----------------------------------------------------------------------------
        if let Some(key) = window.get_input() {
            chat.input(key)?;

            match key {
                Input::Character('c') => break Ok(()),
                Input::KeyResize => {
                    let new_size = window.size();

                    let chat_win = window.new_window(Pos::new(0, 0), new_size)?;
                    chat.reset_window(chat_win);
                }
                _ => {}
            }
        }

        // -----------------------------------------------------------------------------
        //     - Draw -
        // -----------------------------------------------------------------------------
        chat.update();

        window.nap(Duration::from_millis(50));
    }
}

pub fn draw_lines<T>(buffer: &mut ScrollBuffer<Line>, window: &Window<T>, colors: &mut Colors) {
    if buffer.is_dirty() {
        window.erase();
    }
    for line in buffer.lines() {
        let mut pos = window.get_cursor();
        for inst in line.instructions() {
            match inst {
                Instruction::Color(col) => {
                    let pair_id = Colors::get_color_pair(*col);
                    window.set_color(pair_id);
                }
                Instruction::Reset => {
                    let pair_id = Colors::get_color_pair(0);
                    window.set_color(pair_id);
                }
                Instruction::Line(line) => drop(window.print(line)),
                Instruction::Pad(pad) => {
                    let mut pos = window.get_cursor();
                    pos.x += *pad as i32;
                    window.move_cursor(pos);
                }
            }
        }
        pos.y += 1;
        pos.x = 0;
        window.move_cursor(pos);
    }

    window.refresh();
}
