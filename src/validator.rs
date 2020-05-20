use actix_web::HttpRequest;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use base64;

const SIGNATURE_HEADER_NAME: &str = "X-Line-Signature";

pub fn validate(channel_secret: String, request: &HttpRequest, body: &str) -> bool {
    println!(r"
header: {:?}
body: {:?}
",
        &request.headers(),
        &body
    );

    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_varkey(channel_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.input(body.as_bytes());
    let result = mac.result();
    let code_bytes = result.code();
    let computed = base64::encode(&code_bytes);
    println!("{:?}", computed);

    match request.headers().get(SIGNATURE_HEADER_NAME) {
        Some(value) => value.to_str().unwrap() == computed,
        None => false,
    }
}