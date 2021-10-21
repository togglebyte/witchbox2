use std::sync::mpsc;
use std::time::Duration;

use rodio::OutputStream;
use rand::prelude::*;
use anathema::{Color, Colors, Cursor, Input, Instruction, Line, Pos, ScrollBuffer, Size, Sub, Window};
use anyhow::Result;

mod animation;
mod chat_display;
mod event_display;
mod fullscreen_display;
pub mod models;

use chat_display::ChatDisplay;
use event_display::EventDisplay;
use fullscreen_display::FullscreenDisplay;
use models::DisplayMessage;

pub type DisplayEventRx = mpsc::Receiver<models::DisplayMessage>;
pub type DisplayEventTx = mpsc::Sender<models::DisplayMessage>;

const EVENT_HEIGHT: i32 = 10;
const NAP_TIME: u64 = 25;

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
    window.set_cursor_visibility(Cursor::Hide)?;
    window.no_delay(true)?;
    setup_colors();

    let (_stream, sound_output_handle) = OutputStream::try_default()?;

    let (event_size, chat_size) = sizes(window.size());
    let event_win = window.new_window(Pos::new(0, 0), event_size)?;
    let chat_win = window.new_window(Pos::new(0, event_size.height), chat_size)?;
    let mut chat = Display::new(chat_win, ChatDisplay);
    let mut event_disp = Display::new(event_win, EventDisplay::new(sound_output_handle.clone()));
    let fullscreen_win = window.new_window(Pos::new(0, 0), window.size())?;
    let mut fullscreen = Display::new(fullscreen_win, FullscreenDisplay::new(sound_output_handle));

    let mut event_buffer = Vec::with_capacity(1024);

    loop {
        // ---------------------------------------------------------------------
        //     - Incoming events -
        // ---------------------------------------------------------------------
        while let Ok(event) = events.try_recv() {
            chat.handle_message(&event);
            event_disp.handle_message(&event);
            fullscreen.handle_message(&event);

            // Stay under capacity
            if event_buffer.len() == event_buffer.capacity() {
                event_buffer.remove(0);
            }
            event_buffer.push(event);
        }

        // ---------------------------------------------------------------------
        //     - Input handling -
        // ---------------------------------------------------------------------
        if let Some(key) = window.get_input() {
            chat.input(key)?;
            event_disp.input(key)?;
            fullscreen.input(key)?;

            match key {
                Input::Character('c') => break Ok(()),
                Input::KeyResize => {
                    // ---------------------------------------------------------
                    //     - Resize all windows -
                    // ---------------------------------------------------------
                    let (event_size, chat_size) = sizes(window.size());
                    let event_win = window.new_window(Pos::zero(), event_size)?;
                    let chat_win = window.new_window(Pos::new(0, event_size.height), chat_size)?;
                    let fullscreen_win = window.new_window(Pos::zero(), window.size())?;
                    chat.reset_window(chat_win);
                    event_disp.reset_window(event_win);
                    fullscreen.reset_window(fullscreen_win);
                }
                _ => {}
            }
        }

        // ---------------------------------------------------------------------
        //     - Update and draw -
        // ---------------------------------------------------------------------

        while fullscreen.handler.wants_update() && !event_disp.handler.wants_update() {
            fullscreen.update_and_draw()?;
            window.nap(Duration::from_millis(NAP_TIME))?;
            chat.buffer.touch();
        }

        chat.update_and_draw()?;
        event_disp.update_and_draw()?;

        window.nap(Duration::from_millis(NAP_TIME))?;
    }
}

// -----------------------------------------------------------------------------
//     - Display context -
// -----------------------------------------------------------------------------
struct DisplayContext<'a> {
    colors: &'a mut Colors,
    buffer: &'a mut ScrollBuffer<Line>,
    window: &'a Window<Sub>,
}

// -----------------------------------------------------------------------------
//     - Display -
// -----------------------------------------------------------------------------
struct Display<T> {
    window: Window<Sub>,
    buffer: ScrollBuffer<Line>,
    handler: T,
    colors: Colors,
}

impl<T: DisplayHandler> Display<T> {
    pub fn new(window: Window<Sub>, handler: T) -> Self {
        let height = window.size().height as usize;
        let colors = Colors::new(100);
        Self { window, buffer: ScrollBuffer::new(height, 1024), handler, colors }
    }

    pub fn reset_window(&mut self, window: Window<Sub>) {
        self.window = window;
        self.buffer.resize(self.window.size().height as usize);
    }

    pub fn update_and_draw(&mut self) -> Result<()> {
        if self.buffer.is_dirty() {
            self.window.erase()?;
        }
        let context = DisplayContext { colors: &mut self.colors, buffer: &mut self.buffer, window: &mut self.window };
        self.handler.update(context)?;
        self.draw_instructions()?;
        Ok(())
    }

    pub fn input(&mut self, input: Input) -> Result<()> {
        let context = DisplayContext { colors: &mut self.colors, buffer: &mut self.buffer, window: &mut self.window };
        self.handler.input(context, input)
    }

    fn draw_instructions(&mut self) -> Result<()> {
        for line in self.buffer.lines() {
            for inst in line.instructions() {
                match inst {
                    Instruction::Color(col) => {
                        let pair_id = Colors::get_color_pair(*col);
                        self.window.set_color(pair_id)?;
                    }
                    Instruction::Reset => {
                        let pair_id = Colors::get_color_pair(0u32);
                        self.window.set_color(pair_id)?;
                    }
                    Instruction::Line(line) => drop(self.window.print(line)),
                    Instruction::Pad(pad) => {
                        let mut pos = self.window.get_cursor();
                        pos.x += *pad as i32;
                        self.window.move_cursor(pos)?;
                    }
                }
            }

            if line.width() < self.window.size().width as usize {
                self.window.add_char('\n')?;
            }

            // pos.y += 1;
            // pos.x = 0;
            // self.window.move_cursor(pos)?;
        }

        self.window.refresh()?;

        Ok(())
    }

    pub fn handle_message(&mut self, msg: &DisplayMessage) {
        let context = DisplayContext { colors: &mut self.colors, buffer: &mut self.buffer, window: &mut self.window };
        self.handler.handle(context, msg);
    }
}

// -----------------------------------------------------------------------------
//     - Display handler -
// -----------------------------------------------------------------------------
trait DisplayHandler {
    fn update(&mut self, _: DisplayContext) -> Result<()> { Ok(()) }
    fn input(&mut self, context: DisplayContext, input: Input) -> Result<()>;
    fn handle(&mut self, context: DisplayContext, msg: &DisplayMessage);
}
