use crate::error_code::ErrorCode;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, PartialEq)]
pub struct HslColour {
  hue: u32,
  saturation: f32,
  lightness: f32,
}

impl std::convert::From<HslColour> for (f32, f32, f32) {
  fn from(colour: HslColour) -> Self {
    (colour.hue as f32, colour.saturation, colour.lightness)
  }
}

impl std::convert::From<RgbColour> for HslColour {
  fn from(colour: RgbColour) -> Self {
    colour.to_hsl().unwrap_or(HslColour {
      hue: 0,
      saturation: 0.0,
      lightness: 0.0,
    })
  }
}

#[derive(Debug, PartialEq)]
pub struct LabColour {
  lightness: f32,
  a: f32,
  b: f32,
}

impl std::convert::From<LabColour> for (f32, f32, f32) {
  fn from(colour: LabColour) -> Self {
    (colour.lightness as f32, colour.a as f32, colour.b as f32)
  }
}

impl std::convert::From<RgbColour> for LabColour {
  fn from(colour: RgbColour) -> Self {
    colour.to_lab().unwrap_or(LabColour {
      lightness: 0.0,
      a: 0.0,
      b: 0.0,
    })
  }
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct RgbColour {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

impl RgbColour {
  pub fn from_hex(hex: &str) -> Result<RgbColour, String> {
    if !is_valid_hex(hex) {
      return Err(String::from(ErrorCode::InvalidHexCharacter(
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
          _ => Err(String::from(ErrorCode::InvalidHexCharacter(
            hex.to_string(),
          ))),
        }
      }
      length => Err(String::from(ErrorCode::InvalidHexLength(length))),
    }
  }

  pub fn from_colour_name(colour: &str) -> Result<RgbColour, String> {
    if is_valid_colour_name(colour).is_err() | !is_valid_colour_name(colour).unwrap() {
      return Err(String::from(ErrorCode::InvalidColourName(
        colour.to_string(),
      )));
    }

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas");
    let canvas: web_sys::HtmlCanvasElement = canvas
      .unwrap()
      .dyn_into::<web_sys::HtmlCanvasElement>()
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
      Err(_) => Err(String::from(ErrorCode::CanvasError)),
    }
  }

  pub fn get_contrasting_colour(&self) -> RgbColour {
    let yiq: u32 =
      (self.red as u32 * 299 + self.green as u32 * 587 + self.blue as u32 * 114) / 1000;
    match yiq {
      yiq if yiq >= 128 => RgbColour {
        red: 0,
        green: 0,
        blue: 0,
      },
      _ => RgbColour {
        red: 255,
        green: 255,
        blue: 255,
      },
    }
  }

  pub fn to_hex(&self) -> String {
    format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
  }

  pub fn to_hsl(&self) -> Result<HslColour, String> {
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
      Err(_) => Err(String::from(ErrorCode::HslConversionError(format!(
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

fn is_valid_hex(hex: &str) -> bool {
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

#[cfg(test)]
mod tests {
  use super::*;

  mod is_valid_hex {
    use super::*;

    #[test]
    fn six_char() {
      assert!(is_valid_hex("d3d09f"))
    }

    #[test]
    fn three_char() {
      assert!(is_valid_hex("fea"))
    }

    #[test]
    fn six_char_with_prefix() {
      assert!(is_valid_hex("#f34a23"))
    }

    #[test]
    fn three_char_with_prefix() {
      assert!(is_valid_hex("#a4d"))
    }

    #[test]
    fn three_char_invalid() {
      assert!(!is_valid_hex("dkl"))
    }

    #[test]
    fn three_char_with_prefix_invalid() {
      assert!(!is_valid_hex("#9x2"))
    }

    #[test]
    fn six_char_invalid() {
      assert!(!is_valid_hex("dkldsa"))
    }

    #[test]
    fn six_char_with_prefix_invalid() {
      assert!(!is_valid_hex("#9x2444"))
    }

    #[test]
    fn invalid() {
      assert!(!is_valid_hex("fdsfdsfrtre"))
    }
  }

  mod rgb_from_hex {
    use super::*;

    #[test]
    fn six_char() {
      assert_eq!(
        RgbColour::from_hex("F43C8E"),
        Ok(RgbColour {
          red: 244,
          green: 60,
          blue: 142
        })
      );
    }

    #[test]
    fn three_char() {
      assert_eq!(
        RgbColour::from_hex("d15"),
        Ok(RgbColour {
          red: 221,
          green: 17,
          blue: 85
        })
      );
    }

    #[test]
    fn invalid_char() {
      assert_eq!(
        RgbColour::from_hex("F43C8X"),
        Err(String::from(ErrorCode::InvalidHexCharacter(
          "F43C8X".to_string()
        )))
      );
    }

    #[test]
    fn invalid_length() {
      assert_eq!(
        RgbColour::from_hex("F43C"),
        Err(String::from(ErrorCode::InvalidHexLength(4)))
      );
    }
  }

  mod rgb_to_hsl {
    use super::*;

    #[test]
    fn reddish() {
      assert_eq!(
        HslColour::from(RgbColour {
          red: 244,
          green: 43,
          blue: 32
        }),
        HslColour {
          hue: 3,
          saturation: 90.5983,
          lightness: 54.11765
        }
      )
    }

    #[test]
    fn rebeccapurple() {
      assert_eq!(
        HslColour::from(RgbColour::from_hex("663399").unwrap()),
        HslColour {
          hue: 270,
          saturation: 50.000008,
          lightness: 40.0
        }
      )
    }

    #[test]
    fn white() {
      assert_eq!(
        HslColour::from(RgbColour {
          red: 255,
          green: 255,
          blue: 255
        }),
        HslColour {
          hue: 0,
          saturation: 0.0,
          lightness: 100.0
        }
      )
    }

    #[test]
    fn black() {
      assert_eq!(
        HslColour::from(RgbColour {
          red: 0,
          green: 0,
          blue: 0
        }),
        HslColour {
          hue: 0,
          saturation: 0.0,
          lightness: 0.0
        }
      )
    }
  }

  mod rgb_to_lab {
    use super::*;

    #[test]
    fn reddish() {
      assert_eq!(
        LabColour::from(RgbColour {
          red: 244,
          green: 43,
          blue: 32
        }),
        LabColour {
          lightness: 53.020706,
          a: 72.232574,
          b: 55.97896
        }
      )
    }

    #[test]
    fn rebeccapurple() {
      assert_eq!(
        LabColour::from(RgbColour::from_hex("663399").unwrap()),
        LabColour {
          lightness: 32.902435,
          a: 42.89223,
          b: -47.156937
        }
      )
    }

    #[test]
    fn white() {
      assert_eq!(
        LabColour::from(RgbColour {
          red: 255,
          green: 255,
          blue: 255
        }),
        LabColour {
          lightness: 100.0,
          a: 0.0052452087,
          b: -0.010418892
        }
      )
    }

    #[test]
    fn black() {
      assert_eq!(
        LabColour::from(RgbColour {
          red: 0,
          green: 0,
          blue: 0
        }),
        LabColour {
          lightness: 0.0,
          a: 0.0,
          b: 0.0
        }
      )
    }
  }

  mod rgb_contrasting_colour {
    use super::*;

    #[test]
    fn dark_colour() {
      assert_eq!(
        RgbColour::from_hex("054").unwrap().get_contrasting_colour(),
        RgbColour {
          red: 255,
          green: 255,
          blue: 255
        }
      )
    }

    #[test]
    fn light_colour() {
      assert_eq!(
        RgbColour::from_hex("f54").unwrap().get_contrasting_colour(),
        RgbColour {
          red: 0,
          green: 0,
          blue: 0
        }
      )
    }
  }
}
