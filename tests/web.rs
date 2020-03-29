//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate bitarray;
extern crate wasm_bindgen_test;

use bitarray::{BitArray};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_new_with_length() {
    let arr = BitArray::new_with_length(256);
    assert_eq!(arr.length, 256);
    assert_eq!(arr.byte_length, 32);
}

#[wasm_bindgen_test]
fn test_get_bit_empty() {
    let arr = BitArray::new_with_length(32);
    assert_eq!(arr.get_bit(0), 0);
}

#[wasm_bindgen_test]
fn test_set_bit_get_bit() {
    let mut arr = BitArray::new_with_length(32);
    arr.set_bit(0, 1);
    arr.set_bit(1, 0);
    arr.set_bit(2, 1);
    arr.set_bit(3, 1);
    assert_eq!(arr.get_bit(0), 1);
    assert_eq!(arr.get_bit(1), 0);
    assert_eq!(arr.get_bit(2), 1);
    assert_eq!(arr.get_bit(3), 1);
}

#[wasm_bindgen_test]
fn test_set() {
    let mut arr = BitArray::new_with_length(32);
    arr.set(&mut [1, 0, 1, 1]);
    assert_eq!(arr.get_bit(0), 1);
    assert_eq!(arr.get_bit(1), 0);
    assert_eq!(arr.get_bit(2), 1);
    assert_eq!(arr.get_bit(3), 1);
}

#[wasm_bindgen_test]
fn test_subarray() {
    let mut arr = BitArray::new_with_length(32);
    arr.set(&mut [1, 0, 1, 1]);
    let result = arr.subarray(0, 2);
    assert_eq!(result, vec![1, 0]);
}