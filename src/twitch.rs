use std::time::Duration;

use anyhow::{anyhow, Result};
use neotwitch::{BitsEvent, ChannelPoints, ChannelPointsEvent, FollowEvent, Irc, SubscribeEvent, TwitchMessage};
use tinyroute::client::{connect, ClientMessage, TcpClient};
use tinyroute::frame::Frame;
use tokio::time;
use log::error;

const MAX_RETRIES: usize = 500;

#[derive(Debug)]
pub enum Twitch {
    Bits(BitsEvent),
    ChannelEvent(ChannelPoints),
    Follow(FollowEvent),
    Sub(SubscribeEvent),
}

pub async fn start(tx: crate::EventSender) {
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
                error!("Failed to connect: {}", e);

                if reconnect_count > MAX_RETRIES {
                    break;
                }

                time::sleep(Duration::from_secs(reconnect_count as u64)).await;
            }
        }
    }
}

async fn run(tx: crate::EventSender, client: TcpClient) -> Result<()> {
    let (client_tx, client_rx) = connect(client, Some(Duration::from_secs(5 * 60 - 10)));

    let msg = b"chat|sub";
    let framed_message = Frame::frame_message(msg);
    client_tx.send(ClientMessage::Payload(framed_message))?;

    let msg = b"cpoints|sub";
    let framed_message = Frame::frame_message(msg);
    client_tx.send(ClientMessage::Payload(framed_message))?;

    while let Ok(bytes) = client_rx.recv_async().await {
        match serde_json::from_slice::<Irc>(&bytes) {
            Ok(irc_msg) => {
                drop(tx.send(crate::Event::from_irc(irc_msg).into()).await);
            }
            Err(_) => match serde_json::from_slice::<TwitchMessage>(&bytes) {
                Ok(TwitchMessage::Message { data: twitch_msg }) => {
                    let message_topic = twitch_msg.topic.split('.').collect::<Vec<&str>>()[0];

                    match message_topic {
                        "channel-bits-events-v1" => { /* fixme: blend this in to v2 */ }
                        "channel-bits-events-v2" => {
                            let data: BitsEvent = serde_json::from_str(&twitch_msg.message).expect("it's all good");
                            let _ = tx.send(crate::Event::from_bits(data).into()).await;
                        }
                        "channel-bits-badge-unlocks" => {}
                        "channel-points-channel-v1" => {
                            let message_data = serde_json::from_str(&twitch_msg.message).expect("it's all good");
                            match message_data {
                                ChannelPointsEvent::RewardRedeemed { redemption, .. } => {
                                    let _ = tx.send(crate::Event::from_channel_event(redemption).into()).await;
                                }
                                _ => {}
                            }
                        }
                        "following" => {
                            let data: FollowEvent =
                                serde_json::from_str(&twitch_msg.message).expect("it's that good ole json");
                            let _ = tx.send(crate::Event::from_follow(data).into()).await;
                        }
                        "channel-subscribe-events-v1" => {
                            let sub = serde_json::from_str::<SubscribeEvent>(&twitch_msg.message).expect("yay");
                            let _ = tx.send(crate::Event::from_sub(sub).into()).await;
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
