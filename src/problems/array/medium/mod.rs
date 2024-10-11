#![allow(dead_code)]

/// Given an array of integers arr[] and an integer target
/// Return indices of the two numbers such that their sum is equal to the target.
/// Otherwise, we will return {-1, -1}.
pub mod two_sum;

/// Given an array consisting of only 0s, 1s, and 2s.
/// Write a program to in-place sort the array without using inbuilt sort functions.
/// ( Expected: Single pass-O(N) and constant space)
pub mod sort_zero_one_two_array;

/// Given an array of N integers, write a program to return an element
/// that occurs more than N/2 times in the given array.
/// You may consider that such an element always exists in the array
pub mod majority_element;

/// Given an integer array arr, find the contiguous subarray (containing at least one number) which
/// has the largest sum and returns its sum and prints the subarray.
pub mod max_subarray_sum {

    use std::fs::File;
    use std::io::BufReader;

    use crate::utils::input::get_vector_from_buf_reader;

    #[derive(Debug)]
    struct SubArray {
        i: usize,
        j: usize,
    }

    /// Find sum of every subarray
    pub fn brute() {
        let file = File::open("./inputs.txt").unwrap();
        let mut file_reader = BufReader::new(file);

        let arr = get_vector_from_buf_reader::<i32>(&mut file_reader);
        let n = arr.len();
        println!("The input vector is {:?}", arr);

        let mut max_sum = 0;
        let mut sub_array: SubArray = SubArray { i: 0, j: 0 };

        for i in 0..n {
            let mut temp_sum = 0;
            for j in i..n {
                temp_sum += arr[j];
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
}
