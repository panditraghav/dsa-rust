#![allow(dead_code)]
use std::{collections::HashMap, fs::File, io::BufReader};

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

/// Array:- 4 2 2 6 4
/// K: 6
/// Here:- {2^2^6} = K
/// Let's say:- {4^2^2^6} = XR
/// And xor of 4:- X
/// X ^ K = XR
/// Taking Xor k both side
/// (X^K)^K = XR ^ K
/// => X = XR ^ K
/// We know xor of a subarray which is XR, we want a subarray whose XOR is K
/// So by above formula we find X and check if there is a element x exists
/// in the subarray whose XOR is XR, if exists, then from X's index+1 to end
/// Will have XOR k
pub fn optimal() {
    println!("num_of_subarray_with_xor_k:optimal");
    let (arr, k) = get_input();
    println!("Input array: {:?}, k: {}", arr, k);
    let mut xor_subarray_count: usize = 0;
    let mut xor_map: HashMap<i32, usize> = HashMap::new();
    let mut xor: i32 = 0;

    for (i, v) in arr.iter().enumerate() {
        xor ^= *v;
        xor_map.entry(xor).or_insert(i);

        if *v == k {
            xor_subarray_count += 1;
            println!("Subarray {}: {:?}", xor_subarray_count, &arr[i..=i])
        }

        if xor == k {
            xor_subarray_count += 1;
            println!("Subarray {}: {:?}", xor_subarray_count, &arr[0..=i])
        }

        if let Some((_, xi)) = xor_map.get_key_value(&(xor ^ k)) {
            xor_subarray_count += 1;
            println!("Subarray {}: {:?}", xor_subarray_count, &arr[(*xi + 1)..=i])
        }
    }
    println!("Ans: {}", xor_subarray_count);
}
