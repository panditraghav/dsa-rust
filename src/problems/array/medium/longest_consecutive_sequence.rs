use std::fs::File;
use std::io::BufReader;

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap()
}

/// We traverse the array and for each element
/// we search again for all the consecutive elements and increase the count
/// Complexity: O(n^2), since for every element we are traversing the array
/// at max of n times but not more
pub fn brute() {
    println!("longest_consecutive_sequence: brute");
    let arr = get_input();
    println!("The input array is: {:?}", arr);
    let mut longest_subsequence_len = 1;

    for el in &arr {
        let mut subseq_len = 1;
        let mut next_el = *el + 1;

        while arr.iter().any(|x| *x == next_el) {
            subseq_len += 1;
            next_el += 1;
        }
        if subseq_len > longest_subsequence_len {
            longest_subsequence_len = subseq_len;
        }
    }

    println!(
        "The longest subsequence is of length: {}",
        longest_subsequence_len
    );
}

/// Sort the array and find lognest consecutive sequence
pub fn better() {
    println!("longest_consecutive_sequence: better");
    let mut arr = get_input();
    println!("The input array is: {:?}", arr);
    let n = arr.len();
    arr.sort();

    let mut subsequence_len = 1;
    let mut longest_subsequence_len = 1;

    for i in 0..(n - 1) {
        if arr[i] + 1 == arr[i + 1] {
            subsequence_len += 1;
            if subsequence_len > longest_subsequence_len {
                longest_subsequence_len = subsequence_len;
            }
        } else {
            subsequence_len = 1;
        }
    }

    println!(
        "The longest subsequence is of length: {}",
        longest_subsequence_len
    );
}
