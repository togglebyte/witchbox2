use std::collections::VecDeque;

use tinybit::widgets::Text;
use tinybit::{Renderer, ScreenPos, ScreenSize, StdoutTarget, Viewport};

fn message_to_widget(user: &str, message: &str, action: bool) -> Text {
    let text = match action {
        true => format!("* {} {}", user, message),
        false => format!("{} > {}", user, message),
    };
    Text::new(&text, None, None)
}

pub struct Chat {
    lines: VecDeque<(String, String, bool)>,
    widgets: Vec<Text>,
    viewport: Viewport,
    scroll_offset: isize,
}

impl Chat {
    pub fn new(size: ScreenSize) -> Self {
        let mut lines = VecDeque::new();

        for i in 0..70 {
            lines.push_back((format!("{}", i), "hello".into(), false));
        }

        Self {
            lines,
            widgets: Vec::new(),
            viewport: Viewport::new(ScreenPos::zero(), size),
            scroll_offset: 0,
        }
    }

    pub fn rebuild_widgets(&mut self) {
        // Clear all widgets and rebuild the widget this.
        // This is just me being lazy
        self.widgets.clear();

        // Build new widgets
        let offset =
            self.lines.len() - self.max_lines() - self.scroll_offset as usize;
        for line in self.lines.iter().skip(offset) {
            let widget = message_to_widget(&line.0, &line.1, line.2);
            self.widgets.push(widget);
        }

        // Draw the widgets onto the viewport
        for (y, widget) in self.widgets.iter().enumerate() {
            self.viewport.draw_widget(widget, ScreenPos::new(0, y as u16));
        }
    }

    pub fn scroll(&mut self, up: bool, amount: isize) {
        match up {
            true => {
                let max = self.lines.len() as isize - self.max_lines() as isize;
                let amount = (max - self.scroll_offset).min(amount);
                // if self.scroll_offset < max 
                self.scroll_offset += amount
            },
            false => {
                if self.scroll_offset - amount < 0 {
                    self.scroll_offset = 0;
                } else {
                    self.scroll_offset -= amount;
                }
            }
        }
    }

    pub fn max_lines(&self) -> usize {
        self.viewport.size.height as usize
    }

    pub fn new_message(&mut self, nick: String, msg: String, action: bool) {
        self.lines.push_back((nick, msg, action));
        while self.lines.len() > self.max_lines() {
            self.lines.pop_front();
        }

        self.rebuild_widgets();
    }
}

impl super::View for Chat {
    fn draw(&mut self, renderer: &mut Renderer<StdoutTarget>) {
        renderer.render(&mut self.viewport);
        // self.viewport.swap_buffers();
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.viewport.resize(width, height);
    }
}
