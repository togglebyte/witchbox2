use tinybit::events::{Event, Events, KeyCode, KeyEvent};
use tinybit::{term_size, Renderer, ScreenSize, StdoutTarget};

mod animation;
mod channel;
mod chat;
mod input;
mod views;

use input::lines;
use views::View;

enum CurrentView {
    Chat,
    ChannelEvent,
}

pub fn run(events: Events<crate::Event>) {
    let mut filters = crate::events::filter::Filter::new();

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

    let mut current_view = CurrentView::ChannelEvent;

    for event in events {
        match event {
            Event::Tick => match current_view {
                CurrentView::ChannelEvent => match channel_events.animating() {
                    true => channel_events.draw(&mut renderer),
                    false => {
                        current_view = CurrentView::Chat;
                        renderer.clear();
                        chat.full();
                        chat.rebuild_widgets();
                        chat.draw(&mut renderer);
                    }
                },
                CurrentView::Chat => {}
            },
            Event::Key(KeyEvent {
                code: KeyCode::Esc | KeyCode::Char('q'),
                ..
            }) => break,
            Event::Key(KeyEvent { code: kc, .. }) => {
                match kc {
                    KeyCode::Char('k') => chat.scroll(true, 1),
                    KeyCode::Char('j') => chat.scroll(false, 1),
                    KeyCode::Char('d') => chat.reset_scroll(),
                    KeyCode::Char('x') => {
                        chat.new_message(
                            "fancy pants".into(),
                            "Look I have a keyboard".into(),
                            false,
                        );
                    }
                    _ => {}
                }

                if let CurrentView::Chat = current_view {
                    chat.rebuild_widgets();
                    chat.draw(&mut renderer);
                }
            }
            Event::Resize(w, h) => {
                renderer.clear();

                channel_events.resize(w, h);
                chat.resize(w, h);
                chat.rebuild_widgets();
                match current_view {
                    CurrentView::Chat => chat.draw(&mut renderer),
                    CurrentView::ChannelEvent => {}
                }
            }
            Event::User(ev) => {
                if let Some(ev) = filters.filter(ev) {
                    match ev {
                        crate::Event::Chat { nick, msg, action } => {
                            chat.new_message(nick, msg, action);
                            chat.draw(&mut renderer);

                            if let CurrentView::Chat = current_view {
                                chat.rebuild_widgets();
                                chat.draw(&mut renderer);
                            }
                        }
                        crate::Event::ClearChat => {
                            chat.clear();
                            chat.rebuild_widgets();
                            chat.draw(&mut renderer);
                        }
                        crate::Event::Twitch(twitch) => {
                            chat.half();
                            chat.rebuild_widgets();
                            channel_events.event(twitch);
                            current_view = CurrentView::ChannelEvent;
                        }
                        crate::Event::Log(_msg) => {}
                        crate::Event::Quit => break,
                    }
                }
            }
        }
    }
}
