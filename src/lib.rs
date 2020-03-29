mod utils;

extern crate js_sys;
extern crate web_sys;

use std::cmp::{min};
use std::ops::{Index, IndexMut};
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct BitArray {
    buffer: ArrayBuffer,
    u8_: Uint8Array,
    pub length: usize,
    pub byte_length: usize,
}

#[wasm_bindgen]
impl BitArray {

    /// Returns a new BitArray from an ArrayBuffer (with offset and length)
    ///
    /// # Arguments
    ///
    /// * `buf` - ArrayBuffer that will be copied into BitArray
    /// * `offset` - Offset (starting point) into `buf`
    /// * `length` - How much of `buf` to be copied into BitArray
    ///
    /// # Returns
    ///
    /// Returns BitArray based on an offset into an ArrayBuffer
    /// and a length
    pub fn new_with_offset_and_length(buf: ArrayBuffer, offset: usize, length: usize) -> BitArray {
        let end = (offset + (length >> 3)) as u32;
        let buffer = buf.slice_with_end(offset as u32, end);
        let u8_ = Uint8Array::new(&buffer);
        let len = (u8_.byte_length() << 3) as usize;
        let byte_len = u8_.byte_length() as usize;
        BitArray{
            buffer: buffer,
            u8_: u8_,
            length: len,
            byte_length: byte_len,
        }
    }

    /// Builds a BitArray with `length` bits
    ///
    /// # Arguments
    ///
    /// * `length` - The number of bits in BitArray
    ///
    /// # Returns
    ///
    /// Returns an empty BitArray with length `length`
    pub fn new_with_length(length: usize) -> BitArray {
        let buffer = ArrayBuffer::new((length >> 3) as u32);
        let u8_ = Uint8Array::new(&buffer);
        let len = (u8_.byte_length() << 3) as usize;
        let byte_len = u8_.byte_length() as usize;
        BitArray{
            buffer: buffer,
            u8_: u8_,
            length: len,
            byte_length: byte_len,
        }
    }

    /// Get whether the bit at `idx` is set
    ///
    /// # Arguments
    ///
    /// * `idx` - Index in the BitArray
    ///
    /// # Returns
    ///
    /// Returns the bit at index `idx`
    pub fn get_bit(&self, idx: usize) -> u8 {
        let new_idx = (idx >> 3) as u32;
        (self.u8_.get_index(new_idx) >> (7 - (idx & 0x7))) & 1
    }

    /// Set specific bit in the BitArray
    ///
    /// # Arguments
    ///
    /// * `idx` - Index of which bit will be set
    ///
    /// * `val` - The value of the bit that will be set (i.e., 0 or 1)
    pub fn set_bit(&mut self, idx: usize, val: u8) {
        let new_index = (idx >> 3) as u32;
        let shifted = 0x80 >> (idx & 0x7);
        let old_value = self.u8_.get_index(new_index);
        let new_value = if val != 0 {
            old_value | shifted
        } else {
            old_value & !shifted
        };
        self.u8_.set_index(new_index, new_value);
    }

    /// Sets the entire buffer to the array provided
    ///
    /// # Arguments
    ///
    /// * `array` - Array to which the BitArray will be set
    pub fn set(&mut self, array: &mut [u8]) {
        for i in 0..array.len() {
            self.set_bit(i, array[i]);
        }
        self.length = (self.u8_.byte_length() << 3) as usize;
    }

    /// Return a slice from `start` to `stop`
    ///
    /// # Arguments
    ///
    /// * `start` - The (inclusive) start index into the BitArray
    /// * `stop` - The (inclusive) end index into the BitArray
    ///
    /// # Returns
    ///
    /// Vector of 8 bit (1 byte) unsigned integers
    pub fn subarray(&self, start: usize, stop: usize) -> Vec<u8> {
        // Make sure we're within bounds
        let stop = min(self.length, stop);

        // Get each bit and return the vector of bytes
        (start..stop).map(|i| self.get_bit(i)).collect()
    }
}
