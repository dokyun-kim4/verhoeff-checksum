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
        digit
    ))
}

fn get_checksum(input: String) {
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
        [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]
    ];

    let mut digits = [];
    for ch in input.chars() {
        if let Some(digit) = ch.to_digit(10) {
            digits.push(digit as usize);
        } else {
            println!("{} is not a digit!", ch)
        }
    }
}

fn main() {
    // Example usage: Get the permutation of digit 7 at position 3
    let digit = 7;
    let position = 10;
    let permuted_digit = get_permutation(digit, position);
    println!(
        "Permutation of digit {} at position {} is {}",
        digit, position, permuted_digit
    );
}
