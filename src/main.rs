extern crate env_logger;

use actix_web::{/*client::Client, */middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use listenfd::ListenFd;
use serde_json;
use reqwest;
use serde::Deserialize;
use envy;

mod webhook;
mod reply;
mod validator;

#[derive(Deserialize, Debug)]
struct LineConfig {
    channel_access_token: String,
    channel_secret: String
}

const SIGNATURE_HEADER_NAME: &str = "X-Line-Signature";
const REPLY_API_ENDPOINT: &str = "https://api.line.me/v2/bot/message/reply";

async fn handle_webhook_request(req: HttpRequest, /*path: web::Path<webhook::PathInfo>, */body: String) -> impl Responder {
    let line_config = match envy::prefixed("LINE_").from_env::<LineConfig>() {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    };

    let signature = &req.headers().get(SIGNATURE_HEADER_NAME).as_ref().map(|v| v.to_str().unwrap());

    if validator::validate(&line_config.channel_secret, signature, &body) {
        let webhook_request: webhook::Request = serde_json::from_str(&body).unwrap();
        for event in &webhook_request.events {
            match handle_event(event) {
                Some(res) => send_reply(&line_config, &res).await,
                None => (),
            }
        }
    } else {
        println!("invalid request");
    }

    HttpResponse::Ok()
}

fn handle_event(event: &webhook::Event) -> Option<reply::Request> {
    match event {
        webhook::Event::Message { timestamp: _, source: _, reply_token, message } => {
            let reply_messages = match message {
                webhook::Message::Text { id: _, text } if text == "あなたは誰ですか？" => vec![
                    reply::Message::Text { 
                        text: "私は rust で実装された何かです。".to_string(),
                    },
                ],
                webhook::Message::Text { id: _, text } => vec![
                    reply::Message::Text { 
                        text: format!("{} ですね。わかります。", &text).to_string(),
                    },
                ],
                webhook::Message::Location { id: _, title, address: _, latitude, longitude } => {
                    let target_title = match title {
                        Some(t) => t,
                        None => "(不明)",
                    };

                    let text = format!("{} ですね。Google Maps で開きたいですか？", &target_title).to_string();
                    let google_maps_url = format!("https://www.google.com/maps/search/?api=1&query={},{}", latitude, longitude).to_string();
                    vec![
                        reply::Message::Template { 
                            alt_text: "これはだいたいてきすと".to_string(),
                            template: reply::Template::Buttons {
                                text,
                                actions: vec![
                                    reply::Action::Postback {
                                        label: "是非".to_string(),
                                        display_text: Some("お願いします！".to_string()),
                                        data: google_maps_url,
                                    }
                                ]
                            }
                        },
                    ]
                }
                _ => vec![
                    reply::Message::Text { 
                        text: format!("すみません。よくわかりませんのでダンプします。\n{:?}", &message).to_string(),
                    },
                ]
            };

            let reply_request = reply::Request {
                reply_token: reply_token.to_string(),
                messages: reply_messages,
            };

            println!("reply: {:?}", reply_request);

            Some(reply_request)
        },
        webhook::Event::Postback { timestamp: _, source: _, reply_token, postback: webhook::Postback { data }} => {
            let reply_request = reply::Request {
                reply_token: reply_token.to_string(),
                messages: vec![
                    reply::Message::Template { 
                        alt_text: "これはだいたいてきすと".to_string(),
                        template: reply::Template::Buttons {
                            text: data.to_string(),
                            actions: vec![
                                reply::Action::Uri {
                                    uri: data.to_string(),
                                    label: "ぐぐるまぷ".to_string(),
                                }
                            ]
                        }
                    },
                ],
            };
            Some(reply_request)
        },
        _ => {
            println!("not implements. event: {:?}", &event);
            None
        },
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .route("/webhook/{client_id}/{integration_id}", web::post().to(handle_webhook_request))
            .route("/status", web::get().to(|| HttpResponse::Ok()))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}

async fn send_reply(config: &LineConfig, reply_request: &reply::Request) -> () {
    // Proxy に対応してない模様
    // let response = Client::default().post("https://api.line.me/v2/bot/message/reply")
    //    .content_type("application/json")
    //    .header("Authorization", format!("Bearer {}", CHANNEL_ACCESS_TOKEN))
    //    .send_json(&reply_request)
    //    .await;

    let result = reqwest::Client::new()
        .post(REPLY_API_ENDPOINT)
        .bearer_auth(&config.channel_access_token)
        .json(reply_request)
        .send()
        .await;

    println!("response: {:?}", result)
}
