use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

// Expose a simple function to JavaScript
#[wasm_bindgen]
pub fn get_checksum(input: String) -> usize {
    panic_init();
    let d5_cayley_table = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
        [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
        [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
        [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
        [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
        [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
        [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
        [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    ];

    let inv_table = [
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        [0, 4, 3, 2, 1, 5, 6, 7, 8, 9],
    ];

    let mut digits = vec![];
    for ch in input.chars() {
        if let Some(digit) = ch.to_digit(10) {
            digits.push(digit as usize);
        } else {
            println!("{} is not a digit!", ch);
        }
    }
    let mut c = 0;
    for (idx, digit) in digits.iter().rev().enumerate() {
        let p_idx: usize = get_permutation(*digit, idx + 1).expect("REASON");
        c = d5_cayley_table[c][p_idx];
    }
    inv_table[1][c]
}

#[wasm_bindgen]
pub fn validate_checksum(input: String) -> bool {
    panic_init();
    let checksum = get_checksum(input[0..(input.len() - 1)].to_string());
    let last_digit: usize = input.chars().last().unwrap().to_digit(10).unwrap() as usize;
    println!("{}, {}", checksum, last_digit);
    checksum == last_digit
}

fn get_permutation(digit: usize, position: usize) -> Result<usize, String> {
    // Cyclic permutation cycles
    let cycle1 = [1, 5, 8, 9, 4, 2, 7, 0];
    let cycle2 = [3, 6];

    for i in 0..cycle1.len() {
        if cycle1[i] == digit {
            let new_pos: usize = (i + position) % cycle1.len();
            return Ok(cycle1[new_pos]);
        }
    }

    for i in 0..cycle2.len() {
        if cycle2[i] == digit {
            let new_pos: usize = (i + position) % cycle2.len();
            return Ok(cycle2[new_pos]);
        }
    }

    Err(format!(
        "Digit {} is not part of any permutation cycle!",
        digit))
}

fn panic_init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
