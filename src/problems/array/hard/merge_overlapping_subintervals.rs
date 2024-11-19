#![allow(dead_code)]
use std::{fs::File, io::BufReader};

use crate::utils::input::input_matrix_from_buf_reader;

fn get_input() -> Vec<Vec<i32>> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);
    input_matrix_from_buf_reader::<i32>(&mut file_reader)
}

/// First sorting the subinterval array.
/// Iterating over the matrix and checking if the first element of current element
/// is less then or equal to last element of the last element of merged_mat, this means
/// current element is overlapping, then just changing the second element of the overlapping
/// matrix to the last element of current matrix
pub fn optimal() {
    let mut mat: Vec<Vec<i32>> = get_input();
    let n = mat.len();
    println!("Input matrix: {:?}", mat);
    mat.sort();
    println!("Sorted input matrix: {:?}", mat);
    let mut merged_mat: Vec<Vec<i32>> = vec![mat[0].clone()];

    for interval in &mat[1..n] {
        let last = merged_mat.last_mut().unwrap();
        if interval[0] <= last[1] {
            if last[1] < interval[1] {
                last[1] = interval[1];
            }
        } else {
            merged_mat.push(interval.clone());
        }
    }

    println!("Merged matrix is: {:?}", merged_mat);
}
