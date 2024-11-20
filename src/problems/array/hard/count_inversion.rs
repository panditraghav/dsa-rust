#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);
    input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap()
}

pub fn brute() {
    let arr = get_input();
    println!("Input array: {:?}", arr);
    let mut num_count_inverse = 0;

    for i in 0..arr.len() {
        for j in (i + 1)..arr.len() {
            if arr[i] > arr[j] {
                num_count_inverse += 1;
                println!(
                    "Count inversion\n\tarr[{}]: {}, arr[{}]: {}",
                    i, arr[i], j, arr[j]
                );
            }
        }
    }
    println!("Number of count inversion: {}", num_count_inverse);
}
