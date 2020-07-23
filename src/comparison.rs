use crate::colour::RgbColour;
use wasm_bindgen::prelude::*;

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
      &RgbColour {
        red: 255,
        green: 255,
        blue: 255,
      }
      .into(),
      &RgbColour {
        red: 0,
        green: 0,
        blue: 0,
      }
      .into(),
    );
    let actual = Comparison::euclidian_distance(&self.a.into(), &self.b.into());
    let percentage = Comparison::calculate_percentage(actual, max);

    ComparisonResult(actual, percentage)
  }

  pub fn hsl(&self) -> ComparisonResult {
    let max = Comparison::euclidian_distance(
      &RgbColour {
        red: 255,
        green: 255,
        blue: 255,
      }
      .to_hsl()
      .unwrap()
      .into(),
      &RgbColour {
        red: 0,
        green: 0,
        blue: 0,
      }
      .to_hsl()
      .unwrap()
      .into(),
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
      &RgbColour {
        red: 255,
        green: 255,
        blue: 255,
      }
      .to_lab()
      .unwrap()
      .into(),
      &RgbColour {
        red: 0,
        green: 0,
        blue: 0,
      }
      .to_lab()
      .unwrap()
      .into(),
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

    let result = delta_lklsl * delta_lklsl + delta_ckcsc * delta_ckcsc + delta_hkhsh * delta_hkhsh;

    if result < 0.0 {
      0.0
    } else {
      result.sqrt()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod rgb {
    use super::*;

    #[test]
    fn comparison_1() {
      assert_eq!(
        Comparison::new(
          RgbColour {
            red: 3,
            green: 43,
            blue: 234
          },
          RgbColour {
            red: 43,
            green: 54,
            blue: 231
          }
        )
        .rgb(),
        ComparisonResult(41.59327, 90)
      )
    }

    #[test]
    fn comparison_2() {
      assert_eq!(
        Comparison::new(
          RgbColour {
            red: 65,
            green: 123,
            blue: 165
          },
          RgbColour {
            red: 87,
            green: 87,
            blue: 65
          }
        )
        .rgb(),
        ComparisonResult(108.535706, 75)
      )
    }
  }

  mod hsl {
    use super::*;

    #[test]
    fn comparison_1() {
      assert_eq!(
        Comparison::new(
          RgbColour {
            red: 3,
            green: 43,
            blue: 234
          },
          RgbColour {
            red: 43,
            green: 54,
            blue: 231
          }
        )
        .hsl(),
        ComparisonResult(20.142855, 79)
      )
    }

    #[test]
    fn comparison_2() {
      assert_eq!(
        Comparison::new(
          RgbColour {
            red: 65,
            green: 123,
            blue: 165
          },
          RgbColour {
            red: 87,
            green: 87,
            blue: 65
          }
        )
        .hsl(),
        ComparisonResult(148.66129, -49)
      )
    }
  }

  mod lab {
    use super::*;

    #[test]
    fn comparison_1() {
      assert_eq!(
        Comparison::new(
          RgbColour {
            red: 3,
            green: 43,
            blue: 234
          },
          RgbColour {
            red: 43,
            green: 54,
            blue: 231
          }
        )
        .lab(),
        ComparisonResult(2.8480887, 97)
      )
    }

    #[test]
    fn comparison_2() {
      assert_eq!(
        Comparison::new(
          RgbColour {
            red: 65,
            green: 123,
            blue: 165
          },
          RgbColour {
            red: 87,
            green: 87,
            blue: 65
          }
        )
        .lab(),
        ComparisonResult(30.377132, 69)
      )
    }
  }
}
