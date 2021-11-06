//
//                           /[-])//  ___
//                      __ --\ `_/~--|  / \
//                    /_-/~~--~~ /~~~\\_\ /\
//                    |  |___|===|_-- | \ \ \
//  _/~~~~~~~~|~~\,   ---|---\___/----|  \/\-\
//  ~\________|__/   / // \__ |  ||  / | |   | |
//           ,~-|~~~~~\--, | \|--|/~|||  |   | |
//           [3-|____---~~ _--'==;/ _,   |   |_|
//                       /   /\__|_/  \  \__/--/
//                      /---/_\  -___/ |  /,--|
//                      /  /\/~--|   | |  \///
//                     /  / |-__ \    |/
//                    |--/ /      |-- | \
//                   \^~~\\/\      \   \/- _
//                    \    |  \     |~~\~~| \
//                     \    \  \     \   \  | \
//                       \    \ |     \   \    \
//                        |~~|\/\|     \   \   |
//                       |   |/         \_--_- |\
//                       |  /            /   |/\/
//                        ~~             /  /
//                                      |__/
use std::time::Duration;

use tokio::time;

use crate::display::models::DisplayMessage;
use crate::display::DisplayEventTx;
use crate::{Event, EventReceiver};
use crate::audio::{random_sub, random_follow};

mod channel_events;
mod chat;
mod chatfilter;
mod filters;
mod sub;
mod follow;

use channel_events::ChannelPointsTransformer;
use chat::IrcTransformer;
use chatfilter::ChatFilter;
use filters::Filters;
use sub::SubTransformer;
use follow::FollowTransformer;

pub async fn run(mut event_rx: EventReceiver, display_tx: DisplayEventTx) {
    let mut transformers = Transformers::new();
    let mut filters = Filters::new();

    // Receive an event.
    // Queue the display event for sending,
    // however wait N seconds before sending, so more events of the same kind
    // can be grouped

    loop {
        tokio::select! {
            () = time::sleep(Duration::from_secs(1)) => {
                // Drain subs
                for sub in transformers.subs.outstanding() {
                    if let Err(e) = display_tx.send(DisplayMessage::Sub(sub, random_sub())) {
                        log::error!("Failed to send sub to the display: {}", e);
                    }
                }
                if let Some(follows) = transformers.follow.outstanding() {
                    if let Err(e) = display_tx.send(DisplayMessage::Follow(follows, random_follow())) {
                        log::error!("Failed to send follows to the display: {}", e);
                    }
                }
            }
            event = event_rx.recv() => {
                if let Some(event) = event {
                    match event {
                        Event::Chat(irc) => {
                            if let Some(irc) = filters.chat_filter.filter(irc) {
                                let message = transformers.chat.transform(irc);
                                if let Err(e) = display_tx.send(message) {
                                    log::error!("Failed to send message to the display: {}", e);
                                }
                            }
                        }
                        Event::ClearChat => {
                            if let Err(e) = display_tx.send(DisplayMessage::ClearChat) {
                                log::error!("Failed to send clear chat message to the display: {}", e);
                            }
                        }
                        Event::Twitch(twitch) => {
                            match twitch {
                                crate::twitch::Twitch::ChannelEvent(channel_event) => {
                                    if let Some(message) = transformers.channel_events.transform(channel_event) {
                                        if let Err(e) = display_tx.send(message) {
                                            log::error!("Failed to send message to the display: {}", e);
                                        }
                                    }
                                }
                                crate::twitch::Twitch::Sub(sub) => transformers.subs.transform(sub),
                                crate::twitch::Twitch::Follow(follow) => transformers.follow.transform(follow),
                                _ => {} //unimplemented!(),
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

pub struct Transformers {
    chat: IrcTransformer,
    channel_events: ChannelPointsTransformer,
    subs: SubTransformer,
    follow: FollowTransformer,
}

impl Transformers {
    fn new() -> Self {
        Self {
            chat: IrcTransformer::new(),
            channel_events: ChannelPointsTransformer::new(),
            subs: SubTransformer::new(),
            follow: FollowTransformer::new(),
        }
    }
}
