#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);
    input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap()
}

/// **Time Complexity**:- O(n*logn) + O(N)
/// **Space Complexity**:- O(N)
pub fn sort_and_iterate() {
    let mut arr: Vec<i32> = get_input();
    println!("Input array: {:?}", arr);
    arr.sort();
    println!("Sorted array: {:?}", arr);
    let mut el = arr[0];

    for v in &arr {
        if *v != el && *v == el - 1 {
            println!("Missing number is: {}", v);
            println!("Number that appears twice is: {}", el);
            break;
        }
        el += 1;
    }
}
