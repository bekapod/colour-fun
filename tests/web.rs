//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use colour_fun::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn six_char_hex_to_rgb_white() {
    assert_eq!(hex_to_rgb("FFFFFF"), Ok(RgbColour::new(255, 255, 255)));
}

#[wasm_bindgen_test]
fn six_char_hex_to_rgb_pink() {
    assert_eq!(hex_to_rgb("F43C8E"), Ok(RgbColour::new(244, 60, 142)));
}

#[wasm_bindgen_test]
fn three_char_hex_to_rgb_black() {
    assert_eq!(hex_to_rgb("000"), Ok(RgbColour::new(0, 0, 0)));
}

#[wasm_bindgen_test]
fn three_char_hex_to_rgb_pink() {
    assert_eq!(hex_to_rgb("d15"), Ok(RgbColour::new(221, 17, 85)));
}

#[wasm_bindgen_test]
fn invalid_char_hex_to_rgb() {
    assert_eq!(
        hex_to_rgb("F43C8X"),
        Err(JsValue::from(ErrorCode::InvalidHexCharacter))
    );
}

#[wasm_bindgen_test]
fn invalid_length_hex_to_rgb() {
    assert_eq!(
        hex_to_rgb("F43C"),
        Err(JsValue::from(ErrorCode::InvalidHexLength))
    );
}

#[wasm_bindgen_test]
fn dark_hex_code_contrasting_colour() {
    assert_eq!(
        get_contrasting_color_for_hex("054"),
        Ok(JsValue::from(ContrastingColour::White))
    )
}

#[wasm_bindgen_test]
fn light_hex_code_contrasting_colour_is_black() {
    assert_eq!(
        get_contrasting_color_for_hex("f54"),
        Ok(JsValue::from(ContrastingColour::Black))
    )
}

#[wasm_bindgen_test]
fn invalid_hex_code_contrasting_colour_is_white() {
    assert_eq!(
        get_contrasting_color_for_hex("vds"),
        Err(JsValue::from(ErrorCode::InvalidHexCharacter))
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
    assert_eq!(colour_name_to_hex("white"), Ok(String::from("ffffff")))
}

#[wasm_bindgen_test]
fn valid_rebeccapurple_is_hex_663399() {
    assert_eq!(
        colour_name_to_hex("rebeccapurple"),
        Ok(String::from("663399"))
    )
}

#[wasm_bindgen_test]
fn valid_yellow_is_hex_ffff00() {
    assert_eq!(colour_name_to_hex("yellow"), Ok(String::from("ffff00")))
}

#[wasm_bindgen_test]
fn valid_pink_is_hex_ffc0cb() {
    assert_eq!(colour_name_to_hex("pink"), Ok(String::from("ffc0cb")))
}

#[wasm_bindgen_test]
fn invalid_rust_is_hex_000000() {
    assert_eq!(colour_name_to_hex("rust"), Ok(String::from("000000")))
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
