#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use crate::utils::input::input_matrix_from_buf_reader;

fn get_input() -> Vec<Vec<i32>> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);
    input_matrix_from_buf_reader::<i32>(&mut file_reader)
}

/// Creating third array and using two pointers filling the third array
pub fn with_extra_space() {
    println!("merge_two_sorted_array: with extra space");
    let two_arrays: Vec<Vec<i32>> = get_input();
    let arr1 = &two_arrays[0];
    let arr2 = &two_arrays[1];
    println!("First array: {:?}", arr1);
    println!("Second array: {:?}", arr2);
    let mut m_arr: Vec<i32> = vec![0; arr1.len() + arr2.len()];
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            m_arr[k] = arr1[i];
            i += 1;
        } else {
            m_arr[k] = arr2[j];
            j += 1;
        }
        k += 1;
    }
    while i < arr1.len() {
        m_arr[k] = arr1[i];
        i += 1;
        k += 1;
    }
    while j < arr2.len() {
        m_arr[k] = arr2[j];
        j += 1;
        k += 1;
    }

    println!("The merged array is: {:?}", m_arr);
}

/// Move smaller elements to the left array and larger elements to right array.
/// Then sort both arrays
/// # Example:
/// **a1**: {1 4 8 10}
/// **a2**: {2 3 9}
/// **After sort**:-
/// **a1**: {1 2 3 4}
/// **a2**: {8 9 10}, So we need to move 2, 3 to left and 8, 10 to right
/// 2 < 10 => swap(2, 10); ia1--;ia2++
/// 3 < 8 => swap(3, 8); ia1--;ia2++
/// 9 !< 4 => break;
pub fn without_extra_space() {
    println!("merge_two_sorted_array: without extra space");
    let mut two_arrays: Vec<Vec<i32>> = get_input();
    println!("First array: {:?}", two_arrays[0]);
    println!("Second array: {:?}", two_arrays[1]);

    let mut first: usize = two_arrays[0].len() - 1;
    let mut second: usize = 0;

    loop {
        if two_arrays[0][first] >= two_arrays[1][second] {
            let temp = two_arrays[0][first];
            two_arrays[0][first] = two_arrays[1][second];
            two_arrays[1][second] = temp;
            first -= 1;
            second += 1;
        } else {
            break;
        }
    }

    two_arrays[0].sort();
    two_arrays[1].sort();
    println!("First sorted array: {:?}", two_arrays[0]);
    println!("Second sorted array: {:?}", two_arrays[1]);
}
