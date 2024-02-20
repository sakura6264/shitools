use base64::prelude::*;
use base64::Engine;

pub fn base64_encode(input: &[u8]) -> String {
    BASE64_STANDARD.encode(input)
}

pub fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let mut err_colector = String::new();
    match BASE64_STANDARD.decode(input) {
        Ok(v) => return Ok(v),
        Err(e) => {
            err_colector.push_str("BASE64_STANDARD_ERROR: ");
            err_colector.push_str(&e.to_string());
            err_colector.push_str("\n");
        }
    }
    match BASE64_URL_SAFE.decode(input) {
        Ok(v) => return Ok(v),
        Err(e) => {
            err_colector.push_str("BASE64_URL_SAFE_ERROR: ");
            err_colector.push_str(&e.to_string());
            err_colector.push_str("\n");
        }
    }
    match BASE64_STANDARD_NO_PAD.decode(input) {
        Ok(v) => return Ok(v),
        Err(e) => {
            err_colector.push_str("BASE64_STANDARD_NO_PAD_ERROR: ");
            err_colector.push_str(&e.to_string());
            err_colector.push_str("\n");
        }
    }
    match BASE64_URL_SAFE_NO_PAD.decode(input) {
        Ok(v) => return Ok(v),
        Err(e) => {
            err_colector.push_str("BASE64_URL_SAFE_NO_PAD_ERROR: ");
            err_colector.push_str(&e.to_string());
            err_colector.push_str("\n");
        }
    }
    Err(err_colector)
}

pub fn hex_encode(input: &[u8]) -> String {
    hex::encode(input)
}

pub fn hex_decode(input: &str) -> Result<Vec<u8>, String> {
    hex::decode(input).map_err(|err| format!("Error: {}", err))
}

pub fn try_ascii(input: &[u8]) -> String {
    let len = input.len();
    let mut buffer = Vec::with_capacity(len * 2);
    for i in 0..len {
        if input[i] < 0x20 || input[i] > 0x7e {
            // utf8 dot C2B7
            buffer.push(0xC2);
            buffer.push(0xB7);
        } else {
            buffer.push(input[i]);
        }
    }
    return String::from_utf8(buffer).unwrap();
}
