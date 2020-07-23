//! Test suite for the Web and headless browsers.
//! These are generally tests that use canvas / dom API's to get colour objects from css colour names.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use colour_fun::colour::*;
use colour_fun::error_code::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

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
        Err(String::from(ErrorCode::InvalidColourName(
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
