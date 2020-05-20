use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "postback", rename_all = "camelCase")]
    Postback {
        label: String,
        data: String,
        display_text: Option<String>,
    },
    
    #[serde(rename = "uri", rename_all = "camelCase")]
    Uri {
        uri: String,
        label: String,
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Template {
    #[serde(rename = "buttons", rename_all = "camelCase")]
    Buttons {
        text: String,
        actions: Vec<Action>,
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "text", rename_all = "camelCase")]
    Text {
        text: String 
    },
    #[serde(rename = "template", rename_all = "camelCase")]
    Template {
        alt_text: String,
        template: Template,
    },
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub reply_token: String,
    pub messages: Vec<Message>,
}