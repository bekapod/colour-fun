//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use colour_fun::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

// wasm_bindgen_test_configure!(run_in_browser);

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
fn pass() {
    assert_eq!(1 + 1, 2);
}
