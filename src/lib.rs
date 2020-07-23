mod utils;

use std::convert::TryInto;
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
    HslConversionError(String),
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
            ErrorCode::HslConversionError(rgb) => {
                JsValue::from_str(&format!("HSL: could not convert {} to HSL format", rgb))
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct HslColour {
    hue: u32,
    saturation: f32,
    lightness: f32,
}

impl HslColour {
    pub fn new(hue: u32, saturation: f32, lightness: f32) -> HslColour {
        HslColour {
            hue,
            saturation,
            lightness,
        }
    }
}

impl std::convert::Into<(f32, f32, f32)> for HslColour {
    fn into(self) -> (f32, f32, f32) {
        (self.hue as f32, self.saturation, self.lightness)
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct LabColour {
    lightness: f32,
    a: f32,
    b: f32,
}

impl LabColour {
    pub fn new(lightness: f32, a: f32, b: f32) -> LabColour {
        LabColour { lightness, a, b }
    }
}

impl std::convert::Into<(f32, f32, f32)> for LabColour {
    fn into(self) -> (f32, f32, f32) {
        (self.lightness as f32, self.a as f32, self.b as f32)
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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
        if is_valid_colour_name(colour).is_err() | !is_valid_colour_name(colour).unwrap() {
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

    pub fn to_hsl(&self) -> Result<HslColour, JsValue> {
        let red = self.red as f32 / 255.0;
        let green = self.green as f32 / 255.0;
        let blue = self.blue as f32 / 255.0;
        let mut ordered = [red, green, blue];
        ordered.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min = ordered[0];
        let max = ordered[2];
        let delta = max - min;

        let h: f32;
        let hue: Result<u32, std::num::TryFromIntError>;

        let mut lightness: f32 = (max + min) / 2.0;
        let mut saturation: f32;

        match delta {
            val if val == 0.0 => {
                h = 0.0;
                saturation = 0.0;
            }
            val => {
                saturation = val / (1.0 - (2.0 * lightness - 1.0).abs());

                match max {
                    val if (val - red).abs() < 0.001 => {
                        h = ((green - blue) / delta) % 6.0;
                    }
                    val if (val - green).abs() < 0.001 => {
                        h = (blue - red) / delta + 2.0;
                    }
                    _ => {
                        h = (red - green) / delta + 4.0;
                    }
                }
            }
        }

        saturation = (saturation * 100.0).abs();
        lightness = (lightness * 100.0).abs();

        match (h * 60.0).round() as i16 {
            val if val < 0 => {
                hue = (val + 360).try_into();
            }
            val => {
                hue = val.try_into();
            }
        }

        match hue {
            Ok(hue) => Ok(HslColour {
                hue,
                saturation,
                lightness,
            }),
            Err(_) => Err(JsValue::from(ErrorCode::HslConversionError(format!(
                "{:?}",
                self
            )))),
        }
    }

    pub fn to_lab(&self) -> Result<LabColour, JsValue> {
        fn do_weird_thing(val: f32) -> f32 {
            if val > 0.04045 {
                ((val + 0.055) / 1.055).powf(2.4)
            } else {
                val / 12.92
            }
        }

        fn do_other_weird_thing(val: f32) -> f32 {
            if val > 0.008856 {
                val.powf(1.0 / 3.0)
            } else {
                7.787 * val + 16.0 / 116.0
            }
        }

        let red = do_weird_thing(self.red as f32 / 255.0);
        let green = do_weird_thing(self.green as f32 / 255.0);
        let blue = do_weird_thing(self.blue as f32 / 255.0);

        let x = do_other_weird_thing((red * 0.4124 + green * 0.3576 + blue * 0.1805) / 0.95047);
        let y = do_other_weird_thing((red * 0.2126 + green * 0.7152 + blue * 0.0722) / 1.0);
        let z = do_other_weird_thing((red * 0.0193 + green * 0.1192 + blue * 0.9505) / 1.08883);

        Ok(LabColour {
            lightness: 116.0 * y - 16.0,
            a: 500.0 * (x - y),
            b: 200.0 * (y - z),
        })
    }
}

impl std::convert::Into<(f32, f32, f32)> for RgbColour {
    fn into(self) -> (f32, f32, f32) {
        (self.red as f32, self.green as f32, self.blue as f32)
    }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct ComparisonResult(pub f32, pub i32);

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct Comparison {
    a: RgbColour,
    b: RgbColour,
}

impl Comparison {
    pub fn new(a: RgbColour, b: RgbColour) -> Comparison {
        Comparison { a, b }
    }

    pub fn rgb(&self) -> ComparisonResult {
        let max = Comparison::euclidian_distance(
            &RgbColour::new(255, 255, 255).into(),
            &RgbColour::new(0, 0, 0).into(),
        );
        let actual = Comparison::euclidian_distance(&self.a.into(), &self.b.into());
        let percentage = Comparison::calculate_percentage(actual, max);

        ComparisonResult(actual, percentage)
    }

    pub fn hsl(&self) -> ComparisonResult {
        let max = Comparison::euclidian_distance(
            &RgbColour::new(255, 255, 255).to_hsl().unwrap().into(),
            &RgbColour::new(0, 0, 0).to_hsl().unwrap().into(),
        );
        let actual = Comparison::euclidian_distance(
            &self.a.to_hsl().unwrap().into(),
            &self.b.to_hsl().unwrap().into(),
        );
        let percentage = Comparison::calculate_percentage(actual, max);

        ComparisonResult(actual, percentage)
    }

    pub fn lab(&self) -> ComparisonResult {
        let max = Comparison::delta_e(
            &RgbColour::new(255, 255, 255).to_lab().unwrap().into(),
            &RgbColour::new(0, 0, 0).to_lab().unwrap().into(),
        );
        let actual = Comparison::delta_e(
            &self.a.to_lab().unwrap().into(),
            &self.b.to_lab().unwrap().into(),
        );
        let percentage = Comparison::calculate_percentage(actual, max);

        ComparisonResult(actual, percentage)
    }

    fn calculate_percentage(actual: f32, max: f32) -> i32 {
        (100.0 - actual * (100.0 / max)).floor() as i32
    }

    fn euclidian_distance(a: &(f32, f32, f32), b: &(f32, f32, f32)) -> f32 {
        ((b.0 - a.0).powf(2.0) + (b.1 - a.1).powf(2.0) + (b.2 - a.2).powf(2.0)).sqrt()
    }

    fn delta_e(x: &(f32, f32, f32), y: &(f32, f32, f32)) -> f32 {
        let delta_l = x.0 - y.0;
        let delta_a = x.1 - y.1;
        let delta_b = x.2 - y.2;
        let c_a = (x.1 * x.1 + x.2 * x.2).sqrt();
        let c_b = (y.1 * y.1 + y.2 * y.2).sqrt();
        let delta_c = c_a - c_b;
        let mut delta_h = delta_a * delta_a + delta_b * delta_b - delta_c * delta_c;

        if delta_h < 0.0 {
            delta_h = 0.0;
        } else {
            delta_h = delta_h.sqrt();
        }

        let s_c = 1.0 + 0.045 * c_a;
        let s_h = 1.0 + 0.015 * c_a;
        let delta_lklsl = delta_l / 1.0;
        let delta_ckcsc = delta_c / s_c;
        let delta_hkhsh = delta_h / s_h;

        let result =
            delta_lklsl * delta_lklsl + delta_ckcsc * delta_ckcsc + delta_hkhsh * delta_hkhsh;

        if result < 0.0 {
            0.0
        } else {
            result.sqrt()
        }
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
