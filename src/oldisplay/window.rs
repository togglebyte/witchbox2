use std::time::Duration;
use std::cell::RefCell;

use pancurses::Window as PanWindow;
use pancurses::{endwin, initscr, start_color, Attributes, curs_set, COLOR_PAIR, napms};

use super::{Size, Pos, Pair};

thread_local! {
    static MAIN: RefCell<State> = RefCell::new(State::FirstTime);
}

#[derive(Debug, Copy, Clone)]
enum State {
    FirstTime,
    Init,
    Uninit,
}

#[derive(Debug, Copy, Clone)]
pub enum Cursor {
    Hide,
    Normal,
    HighlyVisible,
}

pub struct Main;
pub struct Sub;

pub struct Window<T> {
    inner: PanWindow,
    win_type: T,
}

impl<T> Window<T> {
    pub fn size(&self) -> Size {
        let (y, x) = self.inner.get_max_yx();

        Size {
            width: x,
            height: y
        }
    }

    pub fn new_window(&self, pos: Pos, size: Size) -> Result<Window<Sub>, i32> {
        let inst = Window {
            inner: self.inner.subwin(size.height, size.width, pos.y, pos.x)?,
            win_type: Sub,
        };
        Ok(inst)
    }

    pub fn set_attribute(&self, attribute: impl Into<u32>) {
        self.inner.attrset(attribute.into());
    }

    pub fn set_color(&self, pair: Pair) {
        self.set_attribute(pair);
    }

    pub fn set_attributes(&self, attributes: Attributes) {
        self.inner.attrset(attributes);
    }

    /// Uses `addstr`, **NOT** printw as it is rather unsafe.
    pub fn print(&self, s: impl AsRef<str>) {
        self.inner.addstr(s);
        // self.inner.printw(s);
    }

    pub fn refresh(&self) {
        self.inner.refresh();
    }

    pub fn erase(&self) {
        self.inner.erase();
    }

    pub fn mv(&self, x: i32, y: i32) {
    }

    pub fn add_char(&self, c: char) {
        self.inner.addch(c);
    }

    pub fn enable_scroll(&self) {
        self.inner.scrollok(true);
    }

    pub fn disable_scroll(&self) {
        self.inner.scrollok(false);
    }
}

// -----------------------------------------------------------------------------
//     - Main window -
//     There should only be one of these at a time
// -----------------------------------------------------------------------------
impl Window<Main> {
    pub fn main(no_echo: bool) -> Result<Self, String> {
        if let State::Init = MAIN.with(|win_init| *win_init.borrow()) {
            return Err("Main window already initialized".into());
        }

        let inst = Self {
            inner: initscr(),
            win_type: Main,
        };

        if let State::FirstTime = MAIN.with(|win_init| *win_init.borrow()) {
            start_color();
            if no_echo {
                pancurses::noecho();
            }
        }

        MAIN.with(|win_init| win_init.replace(State::Init));

        Ok(inst)
    }

    pub fn no_delay(&self, no_delay: bool) {
        self.inner.nodelay(no_delay);
    }

    pub fn set_cursor(&self, cursor: Cursor) {
        match cursor {
            Cursor::Hide => curs_set(0),
            Cursor::Normal => curs_set(1),
            Cursor::HighlyVisible => curs_set(2),
        };
    }

    pub fn nap(&self, dur: Duration) {
        napms(dur.as_millis() as i32);
    }
}

impl Drop for Main {
    fn drop(&mut self) {
        MAIN.with(|win_init| win_init.replace(State::Uninit));
        endwin();
    }
}

// -----------------------------------------------------------------------------
//     - Sub window -
// -----------------------------------------------------------------------------
impl Window<Sub> {
    fn new(inner: PanWindow) -> Self {
        Self {
            inner,
            win_type: Sub
        }
    }
}
