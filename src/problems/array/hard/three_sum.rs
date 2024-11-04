#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap()
}

/// Time complexity: O(n^3)
pub fn brute() {
    println!("three_sum: brute");
    let arr = get_input();
    let n = arr.len();
    println!("The input array is:- {:?}", arr);

    let mut ans: HashSet<[i32; 3]> = HashSet::new();

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if arr[i] + arr[j] + arr[k] == 0 {
                    let mut s_zero = [arr[i], arr[j], arr[k]];
                    s_zero.sort();
                    ans.insert(s_zero);
                }
            }
        }
    }

    println!("Set of 3 elements whose sum is zero:- {:?}", ans);
}

/// Hashing
/// We have two loops, and a hashmap containing the elements of the array.
/// We need elements such that arr[i] + arr[j] + arr[el] = 0;
/// So arr[el] = - (arr[i] + arr[j]);
/// And by using this formula we search for this element in hashmap
/// Time complexity: O(n^3)
/// Space complexity: O(n) + O(no_unique_triplet)
pub fn better() {
    println!("three_sum: better");
    let arr = get_input();
    let n = arr.len();
    println!("The input array is:- {:?}", arr);

    let mut ans: HashSet<[i32; 3]> = HashSet::new();
    let mut arr_hash: HashMap<i32, usize> = HashMap::new();

    for (idx, el) in arr.iter().enumerate() {
        arr_hash.insert(*el, idx);
    }

    for i in 0..n {
        for j in (i + 1)..n {
            let third_should_be = -(arr[i] + arr[j]);
            if let Some(val) = arr_hash.get(&third_should_be) {
                if *val != i && *val != j {
                    let mut s_zero = [arr[i], arr[j], third_should_be];
                    s_zero.sort();
                    ans.insert(s_zero);
                }
            }
        }
    }

    println!("Set of 3 elements whose sum is zero:- {:?}", ans);
}
