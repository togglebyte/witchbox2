use tinybit::widgets::{ScrollView, Widget};
use tinybit::{ScreenSize, Pixel};
use chrono::prelude::*;

use super::message_to_widget;
use crate::Event;

pub struct Chat {
    events: Vec<(DateTime<Local>, crate::Event)>,
    pub scroll_view: ScrollView,
}

impl Chat {
    pub fn new(size: ScreenSize) -> Self {
        Self {
            events: Vec::new(),
            scroll_view: ScrollView::new(size, 150),
        }
    }

    pub fn push_event(&mut self, event: crate::Event) {
        while self.events.len() > 150 {
            self.events.remove(0);
        }
        match event {
            // Event::ClearChat => self.clear(),
            Event::Chat(_) 
            | Event::ChatEvent(_) => self.events.push((Local::now(), event)),
            _ => {}
        }
        self.rebuild_widgets();
    }

    pub fn resize(&mut self, size: ScreenSize) {
        self.scroll_view.resize(size);
        self.rebuild_widgets();
    }

    pub fn pixels(&self) -> Vec<Pixel> {
        self.scroll_view.pixels()
    }

    pub fn clear(&mut self) {
        self.scroll_view.clear();
        self.events.clear();
    }

    fn rebuild_widgets(&mut self) {
        let widgets = self.events.iter().map(|(ts, e)| message_to_widget(
            *ts,
            e,
            self.scroll_view.size().width as usize
        )).flatten().collect();
        self.scroll_view.add_widgets(widgets);
    }
}
