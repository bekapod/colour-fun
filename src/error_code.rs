#[derive(Debug)]
pub enum ErrorCode {
  InvalidHexCharacter(String),
  InvalidHexLength(usize),
  InvalidColourName(String),
  CanvasError,
  HslConversionError(String),
}

impl std::convert::From<ErrorCode> for String {
  fn from(error: ErrorCode) -> String {
    match error {
      ErrorCode::InvalidHexCharacter(value) => {
        format!("Invalid: found invalid characters in hex code: {}", value)
      }
      ErrorCode::InvalidHexLength(length) => format!(
        "Invalid: hex code has invalid length: {}. Length must be 3 or 6.",
        length
      ),
      ErrorCode::InvalidColourName(value) => {
        format!("Invalid: {} is not a valid css colour name", value)
      }
      ErrorCode::CanvasError => {
        "Canvas: error occurred while getting image data from canvas".to_string()
      }
      ErrorCode::HslConversionError(rgb) => format!("HSL: could not convert {} to HSL format", rgb),
    }
  }
}
