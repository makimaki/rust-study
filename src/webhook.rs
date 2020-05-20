use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct PathInfo {
    pub client_id: Uuid,
    pub integration_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type")]
pub enum EventSource {
    #[serde(rename="user", rename_all = "camelCase")]
    User { user_id: String },

    #[serde(other)]
    IgnorableTypeEvent,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type")]
pub enum Message {
    #[serde(rename="text", rename_all = "camelCase")]
    Text {
        id: String,
        text: String,
    },
    
    #[serde(rename="location", rename_all = "camelCase")]
    Location {
        id: String,
        title: Option<String>,
        address: Option<String>,
        latitude: f64,
        longitude: f64,
    },

    #[serde(other)]
    IgnorableType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Postback {
    pub data: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag="type")]
pub enum Event {
    #[serde(rename="message", rename_all = "camelCase")]
    Message {
        timestamp: i64,
        source: EventSource,
        reply_token: String,
        message: Message,
    },

    #[serde(rename="postback", rename_all = "camelCase")]
    Postback {
        timestamp: i64,
        source: EventSource,
        reply_token: String,
        postback: Postback,
    },

    #[serde(other)]
    IgnorableType,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub events: Vec<Event>,
}