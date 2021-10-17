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




#[tokio::main]
async fn main() {
    let bytes = HYDRATE.as_bytes();

    let tcp_client = TcpClient::connect("127.0.0.1:6000").await.unwrap();
    let (tx, rx) = connect(tcp_client, None);

    let res = tx.send(ClientMessage::channel_payload(b"cpoints", bytes)); 
    eprintln!("{:?}", res);
}
