mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

#[derive(Debug)]
pub enum ErrorCode {
    InvalidHexCharacter,
    InvalidHexLength,
    CanvasError,
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
            ErrorCode::CanvasError => {
                JsValue::from_str("Canvas: error occurred while getting image data from canvas")
            }
        }
    }
}

#[derive(Debug)]
pub enum ContrastingColour {
    Black,
    White,
}

impl std::convert::From<ContrastingColour> for JsValue {
    fn from(colour: ContrastingColour) -> JsValue {
        match colour {
            ContrastingColour::Black => JsValue::from_str("black"),
            ContrastingColour::White => JsValue::from_str("white"),
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
                hex_pair_to_int(chars[0], chars[1]),
                hex_pair_to_int(chars[2], chars[3]),
                hex_pair_to_int(chars[4], chars[5]),
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
pub fn get_contrasting_color_for_hex(hex: &str) -> Result<JsValue, JsValue> {
    match hex_to_rgb(hex) {
        Ok(RgbColour { red, green, blue }) => {
            let yiq: u32 = (red as u32 * 299 + green as u32 * 587 + blue as u32 * 114) / 1000;
            match yiq {
                yiq if yiq >= 128 => Ok(JsValue::from(ContrastingColour::Black)),
                _ => Ok(JsValue::from(ContrastingColour::White)),
            }
        }
        Err(error) => Err(error),
    }
}

#[wasm_bindgen]
pub fn is_valid_hex(hex: &str) -> bool {
    if hex.is_empty() {
        return false;
    }

    if hex.starts_with('#') {
        return is_valid_hex(&hex[1..]);
    }

    u32::from_str_radix(hex, 16).is_ok()
}

#[wasm_bindgen]
pub fn colour_name_to_hex(colour: &str) -> Result<String, JsValue> {
    let rgb = colour_name_to_rgb(colour);

    match rgb {
        Ok(RgbColour { red, green, blue }) => Ok(format!("{:02x}{:02x}{:02x}", red, green, blue)),
        Err(error) => Err(error),
    }
}

fn hex_pair_to_int(a: char, b: char) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(&format!("{}{}", a, b), 16)
}

fn colour_name_to_rgb(colour: &str) -> Result<RgbColour, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_height(1);
    canvas.set_width(1);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_fill_style(&JsValue::from(colour));
    context.fill_rect(0.0, 0.0, 1.0, 1.0);

    let data = context.get_image_data(0.0, 0.0, 1.0, 1.0);

    match data {
        Ok(data) => {
            let data = data.data();

            Ok(RgbColour {
                red: data[0],
                green: data[1],
                blue: data[2],
            })
        }
        Err(_) => Err(JsValue::from(ErrorCode::CanvasError)),
    }
}
