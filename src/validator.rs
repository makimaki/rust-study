use sha2::Sha256;
use hmac::{Hmac, Mac};
use base64;

pub fn validate(channel_secret: &String, signature: &Option<&str>, body: &str) -> bool {
    info!(r"
signature: {:?}
body: {:?}
",
        &signature,
        &body
    );

    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_varkey(channel_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.input(body.as_bytes());
    let result = mac.result();
    let code_bytes = result.code();
    let computed = base64::encode(&code_bytes);
    info!("{:?}", computed);

    match *signature {
        Some(value) => value == computed,
        None => false,
    }
}