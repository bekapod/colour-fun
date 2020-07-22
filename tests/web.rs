//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use colour_fun::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn six_char_hex_to_rgb_white() {
    assert_eq!(
        RgbColour::from_hex("FFFFFF"),
        Ok(RgbColour::new(255, 255, 255))
    );
}

#[wasm_bindgen_test]
fn six_char_hex_to_rgb_pink() {
    assert_eq!(
        RgbColour::from_hex("F43C8E"),
        Ok(RgbColour::new(244, 60, 142))
    );
}

#[wasm_bindgen_test]
fn three_char_hex_to_rgb_black() {
    assert_eq!(RgbColour::from_hex("000"), Ok(RgbColour::new(0, 0, 0)));
}

#[wasm_bindgen_test]
fn three_char_hex_to_rgb_pink() {
    assert_eq!(RgbColour::from_hex("d15"), Ok(RgbColour::new(221, 17, 85)));
}

#[wasm_bindgen_test]
fn invalid_char_hex_to_rgb() {
    assert_eq!(
        RgbColour::from_hex("F43C8X"),
        Err(JsValue::from(ErrorCode::InvalidHexCharacter(
            "F43C8X".to_string()
        )))
    );
}

#[wasm_bindgen_test]
fn invalid_length_hex_to_rgb() {
    assert_eq!(
        RgbColour::from_hex("F43C"),
        Err(JsValue::from(ErrorCode::InvalidHexLength(4)))
    );
}

#[wasm_bindgen_test]
fn dark_hex_code_contrasting_colour_is_white() {
    assert_eq!(
        RgbColour::from_hex("054").unwrap().get_contrasting_colour(),
        RgbColour::new(255, 255, 255)
    )
}

#[wasm_bindgen_test]
fn light_hex_code_contrasting_colour_is_black() {
    assert_eq!(
        RgbColour::from_hex("f54").unwrap().get_contrasting_colour(),
        RgbColour::new(0, 0, 0)
    )
}

#[wasm_bindgen_test]
fn six_char_valid_hex_is_valid() {
    assert!(is_valid_hex("d3d09f"))
}

#[wasm_bindgen_test]
fn three_char_valid_hex_is_valid() {
    assert!(is_valid_hex("fea"))
}

#[wasm_bindgen_test]
fn six_char_valid_hex_with_prefix_is_valid() {
    assert!(is_valid_hex("#f34a23"))
}

#[wasm_bindgen_test]
fn three_char_valid_hex_with_prefix_is_valid() {
    assert!(is_valid_hex("#a4d"))
}

#[wasm_bindgen_test]
fn three_char_invalid_hex_is_invalid() {
    assert!(!is_valid_hex("dkl"))
}

#[wasm_bindgen_test]
fn three_char_invalid_hex_with_prefix_is_invalid() {
    assert!(!is_valid_hex("#9x2"))
}

#[wasm_bindgen_test]
fn six_char_invalid_hex_with_prefix_is_invalid() {
    assert!(!is_valid_hex("#9x2444"))
}

#[wasm_bindgen_test]
fn invalid_hex_is_invalid() {
    assert!(!is_valid_hex("fdsfdsfrtre"))
}

#[wasm_bindgen_test]
fn valid_white_is_hex_ffffff() {
    assert_eq!(
        RgbColour::from_colour_name("white").unwrap().to_hex(),
        String::from("ffffff")
    )
}

#[wasm_bindgen_test]
fn valid_rebeccapurple_is_hex_663399() {
    assert_eq!(
        RgbColour::from_colour_name("rebeccapurple")
            .unwrap()
            .to_hex(),
        String::from("663399")
    )
}

#[wasm_bindgen_test]
fn valid_yellow_is_hex_ffff00() {
    assert_eq!(
        RgbColour::from_colour_name("yellow").unwrap().to_hex(),
        String::from("ffff00")
    )
}

#[wasm_bindgen_test]
fn valid_pink_is_hex_ffc0cb() {
    assert_eq!(
        RgbColour::from_colour_name("pink").unwrap().to_hex(),
        String::from("ffc0cb")
    )
}

#[wasm_bindgen_test]
fn invalid_rust_is_hex_invalid() {
    assert_eq!(
        RgbColour::from_colour_name("rust"),
        Err(JsValue::from(ErrorCode::InvalidColourName(
            "rust".to_string()
        )))
    )
}

#[wasm_bindgen_test]
fn valid_fff_is_valid_colour() {
    assert!(is_valid_colour("fff"))
}

#[wasm_bindgen_test]
fn valid_000000_is_valid_colour() {
    assert!(is_valid_colour("000000"))
}

#[wasm_bindgen_test]
fn valid_336699_is_valid_colour() {
    assert!(is_valid_colour("336699"))
}

#[wasm_bindgen_test]
fn valid_yellow_is_valid_colour() {
    assert!(is_valid_colour("yellow"))
}

#[wasm_bindgen_test]
fn valid_black_is_valid_colour() {
    assert!(is_valid_colour("black"))
}

#[wasm_bindgen_test]
fn invalid_5fs_is_not_valid_colour() {
    assert!(!is_valid_colour("5fs"))
}

#[wasm_bindgen_test]
fn invalid_rust_is_not_valid_colour() {
    assert!(!is_valid_colour("rust"))
}

#[wasm_bindgen_test]
fn reddish_rgb_to_hsl() {
    assert_eq!(
        RgbColour::new(244, 43, 32).to_hsl(),
        Ok(HslColour::new(3, 90.5983, 54.11765))
    )
}

#[wasm_bindgen_test]
fn rebeccapurple_rgb_to_hsl() {
    assert_eq!(
        RgbColour::from_colour_name("rebeccapurple")
            .unwrap()
            .to_hsl(),
        Ok(HslColour::new(270, 50.000008, 40.0))
    )
}

#[wasm_bindgen_test]
fn white_rgb_to_hsl() {
    assert_eq!(
        RgbColour::new(255, 255, 255).to_hsl(),
        Ok(HslColour::new(0, 0.0, 100.0))
    )
}

#[wasm_bindgen_test]
fn black_rgb_to_hsl() {
    assert_eq!(
        RgbColour::new(0, 0, 0).to_hsl(),
        Ok(HslColour::new(0, 0.0, 0.0))
    )
}

#[wasm_bindgen_test]
fn reddish_rgb_to_lab() {
    assert_eq!(
        RgbColour::new(244, 43, 32).to_lab(),
        Ok(LabColour::new(53.020706, 72.232544, 55.97896))
    )
}

#[wasm_bindgen_test]
fn rebeccapurple_rgb_to_lab() {
    assert_eq!(
        RgbColour::from_colour_name("rebeccapurple")
            .unwrap()
            .to_lab(),
        Ok(LabColour::new(32.902435, 42.89223, -47.156937))
    )
}

#[wasm_bindgen_test]
fn white_rgb_to_lab() {
    assert_eq!(
        RgbColour::new(255, 255, 255).to_lab(),
        Ok(LabColour::new(100.0, 0.0052452087, -0.010418892))
    )
}

#[wasm_bindgen_test]
fn black_rgb_to_lab() {
    assert_eq!(
        RgbColour::new(0, 0, 0).to_lab(),
        Ok(LabColour::new(0.0, 0.0, 0.0))
    )
}
