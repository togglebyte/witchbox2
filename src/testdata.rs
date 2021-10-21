use tinyroute::client::{TcpClient, connect, ClientMessage};

const HYDRATE: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-points-channel-v1.474725923","message":"{\"type\":\"reward-redeemed\",\"data\":{\"timestamp\":\"2021-09-30T08:50:08.899176904Z\",\"redemption\":{\"id\":\"345b5658-6612-403c-b068-8ac45385e88b\",\"user\":{\"id\":\"474725923\",\"login\":\"togglebit\",\"display_name\":\"togglebit\"},\"channel_id\":\"474725923\",\"redeemed_at\":\"2021-09-30T08:50:08.899176904Z\",\"reward\":{\"id\":\"e5a41bd4-3c15-4f29-93df-8b597908c6f2\",\"channel_id\":\"474725923\",\"title\":\"hydrate! (maybe)\",\"prompt\":\"Make me take a sip of water\",\"cost\":100,\"is_user_input_required\":false,\"is_sub_only\":false,\"image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/e5a41bd4-3c15-4f29-93df-8b597908c6f2/4c3adc08-e204-4699-ab5e-c117c659af4c/custom-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/e5a41bd4-3c15-4f29-93df-8b597908c6f2/4c3adc08-e204-4699-ab5e-c117c659af4c/custom-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/e5a41bd4-3c15-4f29-93df-8b597908c6f2/4c3adc08-e204-4699-ab5e-c117c659af4c/custom-4.png\"},\"default_image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/tree-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/tree-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/tree-4.png\"},\"background_color\":\"D9B475\",\"is_enabled\":true,\"is_paused\":false,\"is_in_stock\":false,\"max_per_stream\":{\"is_enabled\":false,\"max_per_stream\":0},\"should_redemptions_skip_request_queue\":false,\"template_id\":\"template:41d5eae8-4deb-4541-b681-ebdcb3125c0f\",\"updated_for_indicator_at\":\"2020-08-17T14:18:40.599453034Z\",\"max_per_user_per_stream\":{\"is_enabled\":false,\"max_per_user_per_stream\":0},\"global_cooldown\":{\"is_enabled\":true,\"global_cooldown_seconds\":120},\"redemptions_redeemed_current_stream\":null,\"cooldown_expires_at\":\"2021-09-30T08:52:08Z\"},\"status\":\"UNFULFILLED\"}}}"}}"#;

const ANON_GIFT_SUB: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-subscribe-events
-v1.474725923","message":"{\"benefit_end_month\":0,\"user_name\":\"ananonymousgifter\",\"display_name\":\"An Anonymous Gifter\"
,\"channel_name\":\"togglebit\",\"user_id\":\"274598607\",\"channel_id\":\"474725923\",\"recipient_id\":\"25269049\",\"recipien
t_user_name\":\"mtothem1337\",\"recipient_display_name\":\"MtotheM1337\",\"time\":\"2021-09-30T08:57:18.912080682Z\",\"sub_mess
age\":{\"message\":\"\",\"emotes\":null},\"sub_plan\":\"1000\",\"sub_plan_name\":\"Channel Subscription (togglebit)\",\"months\
":1,\"context\":\"subgift\",\"is_gift\":true,\"multi_month_duration\":1}"}}
INFO neotwitch::channelpoints | 0027 | 10:57:11 | ChannelPoints   | {"type":"MESSAGE","data":{"topic":"channel-subscribe-events
-v1.474725923","message":"{\"benefit_end_month\":0,\"channel_name\":\"togglebit\",\"channel_id\":\"474725923\",\"recipient_id\"
:\"25269049\",\"recipient_user_name\":\"mtothem1337\",\"recipient_display_name\":\"MtotheM1337\",\"time\":\"2021-09-30T08:57:18
.913971499Z\",\"sub_message\":{\"message\":\"\",\"emotes\":null},\"sub_plan\":\"1000\",\"sub_plan_name\":\"Channel Subscription
 (togglebit)\",\"months\":1,\"context\":\"anonsubgift\",\"is_gift\":true,\"multi_month_duration\":1}"}}"#;

const GIFT_SUB: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-subscribe-events-v1.474725923","message":"{\"benefit_end_month\":0,\"user_name\":\"temporus\",\"display_name\":\"Temporus\",\"channel_name\":\"togglebit\",\"user_id\":\"31515636\",\"channel_id\":\"474725923\",\"recipient_id\":\"86260954\",\"recipient_user_name\":\"bolvarsdad\",\"recipient_display_name\":\"Bolvarsdad\",\"time\":\"2021-09-30T09:10:40.349010399Z\",\"sub_message\":{\"message\":\"\",\"emotes\":null},\"sub_plan\":\"1000\",\"sub_plan_name\":\"Channel Subscription (togglebit)\",\"months\":1,\"context\":\"subgift\",\"is_gift\":true,\"multi_month_duration\":1}"}}"#;

const BITS: &str = r#"{"type":"MESSAGE","data":{"topic":"channel-bits-events-v2.474725923","message":"{\"data\":{\"user_name\":\"sir_klausi\",\"channel_name\":\"togglebit\",\"user_id\":\"209386371\",\"channel_id\":\"474725923\",\"time\":\"2021-09-30T09:13:06.377665427Z\",\"chat_message\":\"uni244\",\"bits_used\":244,\"total_bits_used\":2400,\"is_anonymous\":false,\"context\":\"cheer\",\"badge_entitlement\":null},\"version\":\"1.0\",\"message_type\":\"bits_event\",\"message_id\":\"2c834234-1401-5d94-8192-50b58c4cd56a\"}"}}"#;

const OSLASH: &str = r##"{"type":"MESSAGE","data":{"topic":"channel-points-channel-v1.474725923","message":"{\"type\":\"reward-redeemed\",\"data\":{\"timestamp\":\"2021-10-20T11:42:00.561696412Z\",\"redemption\":{\"id\":\"283753eb-e9ea-47cd-a1ae-be0d20a62f57\",\"user\":{\"id\":\"474725923\",\"login\":\"togglebit\",\"display_name\":\"togglebit\"},\"channel_id\":\"474725923\",\"redeemed_at\":\"2021-10-20T11:42:00.561696412Z\",\"reward\":{\"id\":\"de138038-dc06-4f1e-a576-9b5e42bedb82\",\"channel_id\":\"474725923\",\"title\":\"Work on: Terminal Social Network\",\"prompt\":\"Spend an hour working on a terminal social network\",\"cost\":6000,\"is_user_input_required\":false,\"is_sub_only\":false,\"image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/de138038-dc06-4f1e-a576-9b5e42bedb82/60f46769-5b3e-4961-8f2d-ec59d1924b25/custom-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/de138038-dc06-4f1e-a576-9b5e42bedb82/60f46769-5b3e-4961-8f2d-ec59d1924b25/custom-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/474725923/de138038-dc06-4f1e-a576-9b5e42bedb82/60f46769-5b3e-4961-8f2d-ec59d1924b25/custom-4.png\"},\"default_image\":{\"url_1x\":\"https://static-cdn.jtvnw.net/custom-reward-images/default-1.png\",\"url_2x\":\"https://static-cdn.jtvnw.net/custom-reward-images/default-2.png\",\"url_4x\":\"https://static-cdn.jtvnw.net/custom-reward-images/default-4.png\"},\"background_color\":\"#45415A\",\"is_enabled\":true,\"is_paused\":false,\"is_in_stock\":true,\"max_per_stream\":{\"is_enabled\":false,\"max_per_stream\":1},\"should_redemptions_skip_request_queue\":false,\"template_id\":null,\"updated_for_indicator_at\":\"2021-10-20T11:41:28.525050694Z\",\"max_per_user_per_stream\":{\"is_enabled\":false,\"max_per_user_per_stream\":0},\"global_cooldown\":{\"is_enabled\":false,\"global_cooldown_seconds\":0},\"redemptions_redeemed_current_stream\":null,\"cooldown_expires_at\":null},\"status\":\"UNFULFILLED\"}}}"}}"##;

pub async fn oslash() {
    let bytes = OSLASH.as_bytes();
    send_test(bytes).await;
}

pub async fn hydrate() {
    let bytes = HYDRATE.as_bytes();
    send_test(bytes).await;
}

pub async fn bits() {
    let bytes = BITS.as_bytes();
    send_test(bytes).await;
}

pub async fn gift_sub() {
    let bytes = GIFT_SUB.as_bytes();
    send_test(bytes).await;
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

pub async fn anon_gift_sub() {
    let bytes = ANON_GIFT_SUB.as_bytes();
    send_test(bytes).await;
}

async fn send_test(bytes: &[u8]) {
    let tcp_client = TcpClient::connect("127.0.0.1:6000").await.unwrap();
    let (tx, _rx) = connect(tcp_client, None);

    let _ = tx.send(ClientMessage::channel_payload(b"cpoints", bytes)); 
}
