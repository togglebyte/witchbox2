use tinybit::events::{Event, Events, KeyCode, KeyEvent};
use tinybit::{term_size, Renderer, ScreenSize, StdoutTarget};

mod animation;
mod channel;
mod chat;
mod input;
mod views;

use views::View;

pub fn run(events: Events<crate::Event>) {
    let (width, height) =
        term_size().expect("Can't get the term size? Can't play the game!");

    // Viewport
    let viewport_size = ScreenSize::new(width, height);

    // Renderer
    let stdout_renderer =
        StdoutTarget::new().expect("Failed to enter raw mode");
    let mut renderer = Renderer::new(stdout_renderer);

    let mut chat = chat::Chat::new(viewport_size);
    let mut channel_events = channel::ChannelEvents::new(viewport_size);
    // let mut views: Vec<Box<dyn View>> =
    //     vec![Box::new(chat::Chat::new(viewport_size))];

    for event in events {
        match event {
            Event::Tick => {
                chat.rebuild_widgets();
                chat.draw(&mut renderer);
                // let _ = views.last_mut().map(|c| c.draw(&mut renderer));
            }
            Event::Key(KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q'),
                ..
            }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => match kc {
                KeyCode::Char('k') => {
                    chat.scroll(true, 3);
                    chat.rebuild_widgets();
                },
                KeyCode::Char('j') => {
                    chat.scroll(false, 5);
                    chat.rebuild_widgets();
                }
                _ => {}
            },
            Event::Resize(w, h) => {
                chat.resize(w, h);
                channel_events.resize(w, h);
                // for view in &mut views {
                //     view.resize(w, h);
                // }
                renderer.clear();
            }
            Event::User(ev) => {
                match ev {
                    crate::Event::Chat { nick, msg, action } => {
                        chat.new_message(nick, msg, action)
                    }
                    crate::Event::Twitch(twitch) => {
                        channel_events.event(twitch)
                    }
                    crate::Event::Log(_msg) => {}
                    crate::Event::Quit => break,
                }
                // let _ = views.last_mut().map(|c| c.event(ev));
            }
        }
    }
}
