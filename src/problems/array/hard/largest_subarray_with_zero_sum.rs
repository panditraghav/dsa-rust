#![allow(dead_code)]
use std::{collections::HashMap, fs::File, io::BufReader};

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap()
}

/// Find all subarrays with zero sum and print the largest
pub fn brute() {
    println!("largest_subarray_with_zero_sum:brute");
    let arr: Vec<i32> = get_input();
    let n: usize = arr.len();
    let mut max_len_subarray: usize = 0;

    for i in 0..n {
        let mut subarray_sum: i32 = 0;
        for j in i..n {
            subarray_sum += arr[j];
            if subarray_sum == 0 {
                println!("i: {}, j: {}", i, j);
                let subarray_len = j - i + 1;
                if subarray_len > max_len_subarray {
                    max_len_subarray = subarray_len;
                }
            }
        }
    }
    println!("Ans: {}", max_len_subarray);
}

/// Using hashmap to keep track of sum at indexes.
/// If the current sum that we got is also available in the HashMap,
/// which means:-
/// arr[0:previous_curr_sum_index] + [previous_curr_sum_index:current_index] =
/// current_sum;
/// As we know that arr[0:previous_curr_sum_index] = current_sum, therefore
/// arr[(previous_curr_sum_index + 1):current_index] = 0
pub fn optimal() {
    println!("largest_subarray_with_zero_sum:better");
    let arr: Vec<i32> = get_input();
    let mut max_len_subarray: usize = 0;
    let mut sum_index_map: HashMap<i32, usize> = HashMap::new();
    let mut current_sum: i32 = 0;

    for (i, el) in arr.iter().enumerate() {
        current_sum += *el;

        if let Some(index) = sum_index_map.get(&current_sum) {
            let current_zero_subarr_len = i - *index;
            if current_zero_subarr_len > max_len_subarray {
                max_len_subarray = current_zero_subarr_len;
            }
        } else {
            sum_index_map.insert(current_sum, i);
        }
    }

    println!("Ans: {}", max_len_subarray);
}
