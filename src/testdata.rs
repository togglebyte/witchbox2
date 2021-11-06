use tinyroute::client::{TcpClient, connect, ClientMessage};

const HYDRATE: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-points-channel-v1.474725923","message":"{\"type\":\"reward-redeemed\",\"data\":{\"timestamp\":\"2021-09-30T08:50:08.899176904Z\",\"redemption\":{\"id\":\"345b5658-6612-403c-b068-8ac45385e88b\",\"user\":{\"id\":\"474725923\",\"login\":\"togglebit\",\"display_name\":\"togglebit\"},\"channel_id\":\"474725923\",\"redeemed_at\":\"2021-09-30T08:50:08.899176904Z\",\"reward\":{\"id\":\"e5a41bd4-3c15-4f29-93df-8b597908c6f2\",\"channel_id\":\"474725923\",\"title\":\"hydrate! (maybe)\",\"prompt\":\"Make me take a sip of water\",\"cost\":100,\"is_user_input_required\":false,\"is_sub_only\":false,\"image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/e5a41bd4-3c15-4f29-93df-8b597908c6f2/4c3adc08-e204-4699-ab5e-c117c659af4c/custom-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/e5a41bd4-3c15-4f29-93df-8b597908c6f2/4c3adc08-e204-4699-ab5e-c117c659af4c/custom-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/e5a41bd4-3c15-4f29-93df-8b597908c6f2/4c3adc08-e204-4699-ab5e-c117c659af4c/custom-4.png\"},\"default_image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/tree-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/tree-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/tree-4.png\"},\"background_color\":\"D9B475\",\"is_enabled\":true,\"is_paused\":false,\"is_in_stock\":false,\"max_per_stream\":{\"is_enabled\":false,\"max_per_stream\":0},\"should_redemptions_skip_request_queue\":false,\"template_id\":\"template:41d5eae8-4deb-4541-b681-ebdcb3125c0f\",\"updated_for_indicator_at\":\"2020-08-17T14:18:40.599453034Z\",\"max_per_user_per_stream\":{\"is_enabled\":false,\"max_per_user_per_stream\":0},\"global_cooldown\":{\"is_enabled\":true,\"global_cooldown_seconds\":120},\"redemptions_redeemed_current_stream\":null,\"cooldown_expires_at\":\"2021-09-30T08:52:08Z\"},\"status\":\"UNFULFILLED\"}}}"}}"#;

const SUB: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-subscribe-events-v1.474725923","message":"{\"benefit_end_month\":10,\"user_name\":\"person\",\"display_name\":\"Person\",\"channel_name\":\"togglebit\",\"user_id\":\"274598607\",\"channel_id\":\"474725923\",\"time\":\"2021-09-30T08:57:18.912080682Z\",\"sub_message\":{\"message\":\"here we can put a bunch of words that should show up\",\"emotes\":null},\"sub_plan\":\"1000\",\"sub_plan_name\":\"Channel Subscription (togglebit)\",\"months\":1,\"context\":\"subgift\",\"is_gift\":false,\"multi_month_duration\":1}"}}"#;

const SUB_STREAK: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-subscribe-events-v1.474725923","message":"{\"benefit_end_month\":10,\"streak_months\":5,\"user_name\":\"person\",\"display_name\":\"Person\",\"channel_name\":\"togglebit\",\"user_id\":\"274598607\",\"channel_id\":\"474725923\",\"time\":\"2021-09-30T08:57:18.912080682Z\",\"sub_message\":{\"message\":\"here we can put a bunch of words that should show up\",\"emotes\":null},\"sub_plan\":\"1000\",\"sub_plan_name\":\"Channel Subscription (togglebit)\",\"months\":1,\"context\":\"subgift\",\"is_gift\":false,\"multi_month_duration\":1}"}}"#;

const ANON_GIFT_SUB: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-subscribe-events-v1.474725923","message":"{\"benefit_end_month\":0,\"user_name\":\"ananonymousgifter\",\"display_name\":\"An Anonymous Gifter\",\"channel_name\":\"togglebit\",\"user_id\":\"274598607\",\"channel_id\":\"474725923\",\"recipient_id\":\"25269049\",\"recipient_user_name\":\"mtothem1337\",\"recipient_display_name\":\"MtotheM1337\",\"time\":\"2021-09-30T08:57:18.912080682Z\",\"sub_message\":{\"message\":\"\",\"emotes\":null},\"sub_plan\":\"1000\",\"sub_plan_name\":\"Channel Subscription (togglebit)\",\"months\":1,\"context\":\"subgift\",\"is_gift\":true,\"multi_month_duration\":1}"}}"#;

const GIFT_SUB: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-subscribe-events-v1.474725923","message":"{\"benefit_end_month\":0,\"user_name\":\"temporus\",\"display_name\":\"Temporus\",\"channel_name\":\"togglebit\",\"user_id\":\"31515636\",\"channel_id\":\"474725923\",\"recipient_id\":\"86260954\",\"recipient_user_name\":\"bolvarsdad\",\"recipient_display_name\":\"Bolvarsdad\",\"time\":\"2021-09-30T09:10:40.349010399Z\",\"sub_message\":{\"message\":\"\",\"emotes\":null},\"sub_plan\":\"1000\",\"sub_plan_name\":\"Channel Subscription (togglebit)\",\"months\":1,\"context\":\"subgift\",\"is_gift\":true,\"multi_month_duration\":1}"}}"#;

const BITS: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-bits-events-v2.474725923","message":"{\"data\":{\"user_name\":\"sir_klausi\",\"channel_name\":\"togglebit\",\"user_id\":\"209386371\",\"channel_id\":\"474725923\",\"time\":\"2021-09-30T09:13:06.377665427Z\",\"chat_message\":\"uni244\",\"bits_used\":244,\"total_bits_used\":2400,\"is_anonymous\":false,\"context\":\"cheer\",\"badge_entitlement\":null},\"version\":\"1.0\",\"message_type\":\"bits_event\",\"message_id\":\"2c834234-1401-5d94-8192-50b58c4cd56a\"}"}}"#;

const OSLASH: &str = r##"{"type":"MESSAGE","data":{"topic":"channel-points-channel-v1.474725923","message":"{\"type\":\"reward-redeemed\",\"data\":{\"timestamp\":\"2021-10-20T11:42:00.561696412Z\",\"redemption\":{\"id\":\"283753eb-e9ea-47cd-a1ae-be0d20a62f57\",\"user\":{\"id\":\"474725923\",\"login\":\"togglebit\",\"display_name\":\"togglebit\"},\"channel_id\":\"474725923\",\"redeemed_at\":\"2021-10-20T11:42:00.561696412Z\",\"reward\":{\"id\":\"de138038-dc06-4f1e-a576-9b5e42bedb82\",\"channel_id\":\"474725923\",\"title\":\"Work on: Terminal Social Network\",\"prompt\":\"Spend an hour working on a terminal social network\",\"cost\":6000,\"is_user_input_required\":false,\"is_sub_only\":false,\"image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/de138038-dc06-4f1e-a576-9b5e42bedb82/60f46769-5b3e-4961-8f2d-ec59d1924b25/custom-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/de138038-dc06-4f1e-a576-9b5e42bedb82/60f46769-5b3e-4961-8f2d-ec59d1924b25/custom-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/de138038-dc06-4f1e-a576-9b5e42bedb82/60f46769-5b3e-4961-8f2d-ec59d1924b25/custom-4.png\"},\"default_image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/default-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/default-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/default-4.png\"},\"background_color\":\"#45415A\",\"is_enabled\":true,\"is_paused\":false,\"is_in_stock\":true,\"max_per_stream\":{\"is_enabled\":false,\"max_per_stream\":1},\"should_redemptions_skip_request_queue\":false,\"template_id\":null,\"updated_for_indicator_at\":\"2021-10-20T11:41:28.525050694Z\",\"max_per_user_per_stream\":{\"is_enabled\":false,\"max_per_user_per_stream\":0},\"global_cooldown\":{\"is_enabled\":false,\"global_cooldown_seconds\":0},\"redemptions_redeemed_current_stream\":null,\"cooldown_expires_at\":null},\"status\":\"UNFULFILLED\"}}}"}}"##;

const CHAT: &[u8] = b"@badge-info=subscriber/18;badges=broadcaster/1,subscriber/3009;client-nonce=a39a735668114631c13778e1befc3df9;color=#5F9EA0;display-name=togglebit;emotes=;first-msg=0;flags=;id=20c10444-e920-478c-aa8d-f39640c19b19;mod=0;room-id=474725923;subscriber=1;tmi-sent-ts=1635955760328;turbo=0;user-id=474725923;user-type= :togglebit!togglebit@togglebit.tmi.twitch.tv PRIVMSG #togglebit :test and then some linebreaksaresilly\r\n";

const CHAT_ACTION: &[u8] = b"@badge-info=subscriber/18;badges=broadcaster/1,subscriber/3009;color=#5F9EA0;display-name=togglebit;emotes=;first-msg=0;flags=;id=53fea765-f9d3-4afc-aafd-f6aa02962edf;mod=0;room-id=474725923;subscriber=1;tmi-sent-ts=1636026223263;turbo=0;user-id=474725923;user-type= :togglebit!togglebit@togglebit.tmi.twitch.tv PRIVMSG #togglebit :\x01ACTION does something\x01\r\n";

const FOLLOW: &str = r#"{"type":"MESSAGE","data":{"topic":"following.474725923","message":"{\"display_name\":\"RandomUser\",\"username\":\"randomuser\",\"user_id\":\"100819325\"}"}}"#;

// INFO neotwitch::channelpoints | 47890 | 11:54:41 | {"type":"MESSAGE","data":{"topic":"following.474725923","message":"{\"display_name\":\"BotDoodah\",\"username\":\"botdoodah\",\"user_id\":\"100819325\"}"}}

pub async fn oslash() {
    let bytes = OSLASH.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn hydrate() {
    let bytes = HYDRATE.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn bits() {
    let bytes = BITS.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn gift_sub() {
    let bytes = GIFT_SUB.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn sub() {
    let bytes = SUB.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn sub_streak() {
    let bytes = SUB_STREAK.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn anon_gift_sub() {
    let bytes = ANON_GIFT_SUB.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn follow() {
    let bytes = FOLLOW.as_bytes();
    send_twich_event(bytes).await;
}

pub async fn chat() {
    send_chat(CHAT).await;
}

pub async fn action() {
    send_chat(CHAT_ACTION).await;
}

async fn send_twich_event(bytes: &[u8]) {
    let tcp_client = TcpClient::connect("127.0.0.1:6000").await.unwrap();
    let (tx, _rx) = connect(tcp_client, None);

    let _ = tx.send(ClientMessage::channel_payload(b"cpoints", bytes)); 
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

async fn send_chat(bytes: &[u8]) {
    let tcp_client = TcpClient::connect("127.0.0.1:6000").await.unwrap();
    let (tx, _rx) = connect(tcp_client, None);

    let _ = tx.send(ClientMessage::channel_payload(b"chat", bytes)); 
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
}
