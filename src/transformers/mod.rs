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

use crate::display::DisplayEventTx;
use crate::{Event, EventReceiver};
use crate::display::models::DisplayMessage;

mod chat;
mod chatfilter;
mod filters;
mod channel_events;

use channel_events::ChannelPointsTransformer;
use chat::IrcTransformer;
use chatfilter::ChatFilter;
use filters::Filters;

pub async fn run(mut event_rx: EventReceiver, display_tx: DisplayEventTx) {
    let mut transformers = Transformers::new();
    let mut filters = Filters::new();

    // Receive an event.
    // Queue the display event for sending,
    // however wait N seconds before sending, so more events of the same kind
    // can be grouped
    while let Some(event) = event_rx.recv().await {
        match event {
            Event::Chat(irc) => {
                if let Some(irc) = filters.chat_filter.filter(irc) {
                    let message = transformers.chat.transform(irc);
                    display_tx.send(DisplayMessage::Chat(message));
                }
            }
            Event::ClearChat => drop(display_tx.send(DisplayMessage::ClearChat)),
            Event::Twitch(twitch) => {
                match twitch {
                    crate::twitch::Twitch::ChannelEvent(channel_event) => {
                        let message = transformers.channel_events.transform(channel_event);
                        display_tx.send(DisplayMessage::ChannelPoints(message));
                    }
                    _ => unimplemented!(),
                }
            }
            _ => {}
        }
    }
}

pub struct Transformers {
    chat: IrcTransformer,
    channel_events: ChannelPointsTransformer,
}

impl Transformers {
    fn new() -> Self {
        Self { 
            chat: IrcTransformer::new(),
            channel_events: ChannelPointsTransformer::new(),
        }
    }
}
