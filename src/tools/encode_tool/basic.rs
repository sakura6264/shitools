use base64::prelude::*;

pub fn base64_encode_std(input: &str) -> String {
    BASE64_STANDARD.encode(input.as_bytes())
}
pub fn base64_decode_std(input: &str) -> String {
    match BASE64_STANDARD.decode(input) {
        Ok(v) => String::from_utf8(v).unwrap(),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn base64_encode_url(input: &str) -> String {
    BASE64_URL_SAFE.encode(input.as_bytes())
}
pub fn base64_decode_url(input: &str) -> String {
    match BASE64_URL_SAFE.decode(input) {
        Ok(v) => String::from_utf8(v).unwrap(),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn base64_encode_std_no_pad(input: &str) -> String {
    BASE64_STANDARD_NO_PAD.encode(input.as_bytes())
}
pub fn base64_decode_std_no_pad(input: &str) -> String {
    match BASE64_STANDARD_NO_PAD.decode(input) {
        Ok(v) => String::from_utf8(v).unwrap(),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn base64_encode_url_no_pad(input: &str) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(input.as_bytes())
}
pub fn base64_decode_url_no_pad(input: &str) -> String {
    match BASE64_URL_SAFE_NO_PAD.decode(input) {
        Ok(v) => String::from_utf8(v).unwrap(),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn url_encode(input: &str) -> String {
    urlencoding::encode(input).into_owned()
}
pub fn url_decode(input: &str) -> String {
    match urlencoding::decode(input) {
        Ok(v) => v.into_owned(),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_encode(input: &str) -> String {
    hex::encode(input)
}
pub fn hex_decode(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => String::from_utf8(v).unwrap(),
        Err(e) => format!("Error: {}", e),
    }
}
