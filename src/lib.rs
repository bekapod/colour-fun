mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
pub enum ErrorCode {
    InvalidHexCharacter(String),
    InvalidHexLength(usize),
    InvalidColourName(String),
    CanvasError,
}

impl std::convert::From<ErrorCode> for JsValue {
    fn from(error: ErrorCode) -> JsValue {
        match error {
            ErrorCode::InvalidHexCharacter(value) => JsValue::from_str(&format!(
                "Invalid: found invalid characters in hex code: {}",
                value
            )),
            ErrorCode::InvalidHexLength(length) => JsValue::from_str(&format!(
                "Invalid: hex code has invalid length: {}. Length must be 3 or 6.",
                length
            )),
            ErrorCode::InvalidColourName(value) => JsValue::from_str(&format!(
                "Invalid: {} is not a valid css colour name",
                value
            )),
            ErrorCode::CanvasError => {
                JsValue::from_str("Canvas: error occurred while getting image data from canvas")
            }
        }
    }
}

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

    pub fn from_hex(hex: &str) -> Result<RgbColour, JsValue> {
        if !is_valid_hex(hex) {
            return Err(JsValue::from(ErrorCode::InvalidHexCharacter(
                hex.to_string(),
            )));
        }

        match hex.len() {
            3 => {
                let chars: Vec<char> = hex.chars().collect();
                RgbColour::from_hex(&format!(
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
                    _ => Err(JsValue::from(ErrorCode::InvalidHexCharacter(
                        hex.to_string(),
                    ))),
                }
            }
            length => Err(JsValue::from(ErrorCode::InvalidHexLength(length))),
        }
    }

    pub fn from_colour_name(colour: &str) -> Result<RgbColour, JsValue> {
        if !is_valid_colour_name(colour).is_ok() | !is_valid_colour_name(colour).unwrap() {
            return Err(JsValue::from(ErrorCode::InvalidColourName(
                colour.to_string(),
            )));
        }

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.create_element("canvas")?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
        canvas.set_height(1);
        canvas.set_width(1);
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
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

    pub fn get_contrasting_colour(&self) -> RgbColour {
        let yiq: u32 =
            (self.red as u32 * 299 + self.green as u32 * 587 + self.blue as u32 * 114) / 1000;
        match yiq {
            yiq if yiq >= 128 => RgbColour::new(0, 0, 0),
            _ => RgbColour::new(255, 255, 255),
        }
    }

    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
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
pub fn is_valid_colour(color: &str) -> bool {
    let is_hex = is_valid_hex(color);
    let is_name = is_valid_colour_name(color);

    match is_name {
        Ok(is_name_value) => is_hex | is_name_value,
        Err(_) => is_hex,
    }
}

fn hex_pair_to_int(a: char, b: char) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(&format!("{}{}", a, b), 16)
}

fn is_valid_colour_name(colour: &str) -> Result<bool, JsValue> {
    let o = web_sys::HtmlOptionElement::new()?;
    let s = o.style();
    s.set_property("color", colour)?;

    println!("{:?}", s.get_property_value("color"));

    Ok(s.get_property_value("color")? == colour)
}
