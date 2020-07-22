mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub struct RgbColour {
    red: u8,
    green: u8,
    blue: u8,
}

impl RgbColour {
    pub fn new(red: u8, green: u8, blue: u8) -> RgbColour {
        RgbColour { red, green, blue }
    }
}

pub enum ErrorCode {
    InvalidHexCharacter,
    InvalidHexLength,
}

impl std::convert::From<ErrorCode> for JsValue {
    fn from(error: ErrorCode) -> JsValue {
        match error {
            ErrorCode::InvalidHexCharacter => {
                JsValue::from_str("Invalid: found invalid characters in hex code")
            }
            ErrorCode::InvalidHexLength => {
                JsValue::from_str("Invalid: hex code has invalid length (must be 3 or 6)")
            }
        }
    }
}

#[wasm_bindgen]
pub fn hex_to_rgb(hex: &str) -> Result<RgbColour, JsValue> {
    match hex.len() {
        3 => {
            let chars: Vec<char> = hex.chars().collect();
            hex_to_rgb(&format!(
                "{r}{r}{g}{g}{b}{b}",
                r = chars[0],
                g = chars[1],
                b = chars[2]
            ))
        }
        6 => {
            let chars: Vec<char> = hex.chars().collect();
            let parsed = (
                hex_str_to_int(chars[0], chars[1]),
                hex_str_to_int(chars[2], chars[3]),
                hex_str_to_int(chars[4], chars[5]),
            );
            match parsed {
                (Ok(red), Ok(green), Ok(blue)) => Ok(RgbColour { red, green, blue }),
                _ => Err(JsValue::from(ErrorCode::InvalidHexCharacter)),
            }
        }
        _ => Err(JsValue::from(ErrorCode::InvalidHexLength)),
    }
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, colour-fun!");
}

fn hex_str_to_int(a: char, b: char) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(&format!("{}{}", a, b), 16)
}
