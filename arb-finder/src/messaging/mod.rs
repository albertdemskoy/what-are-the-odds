use serde::Serialize;

use crate::local_env::MY_ENV;

const BASE_ENDPOINT: &str = "https://discord.com/api/";
const API_KEY: &str = MY_ENV.discord_api_key;
const CHANNEL_ID: &str = MY_ENV.discord_channel_id

#[derive(Serialize)]
struct DiscordMessage {
    content: String,
}

pub fn send_message(msg: &str) {
    let send_message_endpoint = format!("channels/{CHANNEL_ID}/messages");
    let full_url = BASE_ENDPOINT.to_owned() + &send_message_endpoint;

    let message_body = DiscordMessage {
        content: msg.to_string(),
    };

    let client = reqwest::blocking::Client::new();
    client
        .post(full_url)
        .header("Authorization", format!("Bot {}", API_KEY))
        .form(&message_body)
        .send();
}
