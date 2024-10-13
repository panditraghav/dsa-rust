use std::fs::File;
use std::io::BufReader;

use crate::utils::input::input_vector_from_buf_reader;

#[derive(Debug)]
struct SubArray {
    i: usize,
    j: usize,
}

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_vector_from_buf_reader::<i32>(&mut file_reader)
}

/// Find sum of every subarray
pub fn brute() {
    let arr = get_input();
    let n = arr.len();
    println!("The input vector is {:?}", arr);

    let mut max_sum = 0;
    let mut sub_array: SubArray = SubArray { i: 0, j: 0 };

    for i in 0..n {
        let mut temp_sum = 0;
        for (j, j_el) in arr[i..n].iter().enumerate() {
            temp_sum += *j_el;
            if temp_sum > max_sum {
                max_sum = temp_sum;
                sub_array.i = i;
                sub_array.j = j;
            }
        }
    }
    println!(
        "The maximum subarray sum is: {}\n The subarray is: {:?}",
        max_sum, sub_array
    );
}

/// Kadane's Algorithm
/// Whenever sum becomes -ve we make it 0 since, that won't add to subarray
/// If we add -ve then sum will decrease
pub fn optimal() {
    let arr = get_input();
    println!("Input array is: {:?}", arr);

    let mut max_sum = arr[0];
    let mut sum = 0;

    for el in &arr {
        sum += *el;

        if sum > max_sum {
            max_sum = sum;
        }
        if sum < 0 {
            sum = 0;
        }
    }
    println!("Maximum subarray sum is: {}", max_sum);
}
