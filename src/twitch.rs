use std::sync::mpsc;
use std::time::Duration;

use neotwitch::{BitsEvent, Irc, TwitchMessage, ChannelPointsEvent, ChannelPoints};
use tinybit::events::Event;
use tinyroute::client::{connect, ClientMessage, TcpClient};
use tinyroute::frame::Frame;
use tokio::time;

use anyhow::{anyhow, Result};

const MAX_RETRIES: usize = 5;

type Sendy = mpsc::Sender<Event<crate::Event>>;

pub enum Twitch {
    Bits(BitsEvent),
    ChannelEvent(ChannelPoints),
}

pub async fn start(tx: Sendy) {
    let mut reconnect_count = 0;
    loop {
        reconnect_count += 1;

        let tx = tx.clone();
        match TcpClient::connect("127.0.0.1:6000").await {
            Ok(c) => {
                reconnect_count = 0;
                match run(tx, c).await {
                    Ok(()) => {}
                    Err(_) => {}
                }
            }
            Err(e) => {
                let _ = tx.send(Event::User(crate::Event::Log(format!(
                    "Failed to connect: {}",
                    e
                ))));

                if reconnect_count > MAX_RETRIES {
                    break;
                }

                time::sleep(Duration::from_secs(reconnect_count as u64)).await;
            }
        }
    }
}

async fn run(tx: Sendy, client: TcpClient) -> Result<()> {
    let (client_tx, mut client_rx) =
        connect(client, Some(Duration::from_secs(5 * 60 - 10)));

    let msg = b"chat|sub";
    let framed_message = Frame::frame_message(msg);
    client_tx.send(ClientMessage::Payload(framed_message)).await?;

    let msg = b"cpoints|sub";
    let framed_message = Frame::frame_message(msg);
    client_tx.send(ClientMessage::Payload(framed_message)).await?;

    while let Some(bytes) = client_rx.recv().await {
        match serde_json::from_slice::<Irc>(&bytes) {
            Ok(irc_msg) => {
                drop(tx.send(crate::Event::from_irc(irc_msg).into()))
            }
            Err(_) => match serde_json::from_slice::<TwitchMessage>(&bytes) {
                Ok(TwitchMessage::Message { data: twitch_msg }) => {
                    let message_topic =
                        twitch_msg.topic.split('.').collect::<Vec<&str>>()[0];

                    match message_topic {
                        "channel-bits-events-v1" => { /* fixme: blend this in to v2 */ }
                        "channel-bits-events-v2" => {
                            let data: BitsEvent = serde_json::from_str(&twitch_msg.message).expect("it's all good");
                            // let ui_data = models::BitsEvent {
                            //     user_name: data.data.user_name,
                            //     bits: data.data.bits_used,
                            // };
                            let _ = tx.send(crate::Event::from_bits(data).into());
                        }
                        "channel-bits-badge-unlocks" => {}
                        "channel-points-channel-v1" => {
                            let message_data = serde_json::from_str(&twitch_msg.message).expect("it's all good");
                            match message_data {
                                ChannelPointsEvent::RewardRedeemed { redemption, .. } => {
                                    let _ = tx.send(crate::Event::from_channel_event(redemption).into());
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                Ok(_) => {} // ignore for now
                Err(_) => return Err(anyhow!("Neither IRC nor Twitch event")),
            },
        }
    }

    Ok(())
}
