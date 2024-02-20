use encoding::all::*;
use encoding::{DecoderTrap, EncoderTrap, Encoding};

pub fn utf8_to_hex(input: &str) -> String {
    match UTF_8.encode(input, EncoderTrap::Strict) {
        Ok(v) => hex::encode(v),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_to_utf8(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => match UTF_8.decode(&v, DecoderTrap::Strict) {
            Ok(v) => v,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}
pub fn utf16le_to_hex(input: &str) -> String {
    match UTF_16LE.encode(input, EncoderTrap::Strict) {
        Ok(v) => hex::encode(v),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_to_utf16le(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => match UTF_16LE.decode(&v, DecoderTrap::Strict) {
            Ok(v) => v,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}
pub fn utf16be_to_hex(input: &str) -> String {
    match UTF_16BE.encode(input, EncoderTrap::Strict) {
        Ok(v) => hex::encode(v),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_to_utf16be(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => match UTF_16BE.decode(&v, DecoderTrap::Strict) {
            Ok(v) => v,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}
pub fn gbk_to_hex(input: &str) -> String {
    match GBK.encode(input, EncoderTrap::Strict) {
        Ok(v) => hex::encode(v),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_to_gbk(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => match GBK.decode(&v, DecoderTrap::Strict) {
            Ok(v) => v,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}
pub fn shiftjis_to_hex(input: &str) -> String {
    match EUC_JP.encode(input, EncoderTrap::Strict) {
        Ok(v) => hex::encode(v),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_to_shiftjis(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => match EUC_JP.decode(&v, DecoderTrap::Strict) {
            Ok(v) => v,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}
pub fn big5_to_hex(input: &str) -> String {
    match BIG5_2003.encode(input, EncoderTrap::Strict) {
        Ok(v) => hex::encode(v),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_to_big5(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => match BIG5_2003.decode(&v, DecoderTrap::Strict) {
            Ok(v) => v,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}
pub fn ascii_to_hex(input: &str) -> String {
    match ASCII.encode(input, EncoderTrap::Strict) {
        Ok(v) => hex::encode(v),
        Err(e) => format!("Error: {}", e),
    }
}
pub fn hex_to_ascii(input: &str) -> String {
    match hex::decode(input) {
        Ok(v) => match ASCII.decode(&v, DecoderTrap::Strict) {
            Ok(v) => v,
            Err(e) => format!("Error: {}", e),
        },
        Err(e) => format!("Error: {}", e),
    }
}
