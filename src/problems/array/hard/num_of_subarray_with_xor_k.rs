#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use crate::utils::input::{input_num_from_buf_reader, input_vector_from_buf_reader};

fn get_input() -> (Vec<i32>, i32) {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    (
        input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap(),
        input_num_from_buf_reader(&mut file_reader).unwrap(),
    )
}

/// Find all subarrays with xor k
pub fn brute() {
    println!("num_of_subarray_with_xor_k:brute");
    let (arr, k) = get_input();
    println!("Input array: {:?}, k: {}", arr, k);
    let n: usize = arr.len();
    let mut xor_subarray_count: usize = 0;

    for i in 0..n {
        let mut subarray_xor: i32 = 0;
        for j in i..n {
            subarray_xor ^= arr[j];
            if subarray_xor == k {
                println!("i: {}, j: {}, subarr: {:?}", i, j, &arr[i..(j + 1)]);
                xor_subarray_count += 1;
            }
        }
    }
    println!("Ans: {}", xor_subarray_count);
}
