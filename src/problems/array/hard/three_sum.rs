#![allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use std::cmp::Ordering::{Equal, Greater, Less};

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

/// We use 3 pointers, left, mid, end
/// First we sort the array
/// Initially: left = 0; mid = left + 1; end = n - 1;
/// We check sum arr[left] + arr[mid] + arr[end] == 0 ?
/// If yes: put it in the set,
/// if greater: We need to decrease the sum so end-- until arr[end] != previous arr[end]
/// if less: We need to increase sum so mid++ until arr[mid] != previous arr[mid]
/// if mid <= less: increase left; left++ until until arr[left] != previous arr[left]
/// and mid = left + 1; repeat this until left < n - 2;
pub fn optimal() {
    println!("three_sum: optimal");
    let mut arr = get_input();
    let n = arr.len();
    arr.sort();
    println!("The sorted input array is:- {:?}", arr);

    let mut ans: HashSet<[i32; 3]> = HashSet::new();

    let mut start = 0;
    let mut mid = start + 1;
    let mut end = n - 1;

    while start < n - 3 {
        if mid >= end {
            let el_start = arr[start];
            while arr[start] == el_start {
                start += 1;
            }
            mid = start + 1;
            end = n - 1;
        }
        let sum = arr[start] + arr[mid] + arr[end];
        println!("{},{},{}, sum: {}", start, mid, end, sum);
        match sum.cmp(&0) {
            Equal => {
                ans.insert([arr[start], arr[mid], arr[end]]);
                let el_mid = arr[mid];
                while arr[mid] == el_mid {
                    mid += 1;
                }
                let el_end = arr[end];
                while arr[end] == el_end {
                    end -= 1;
                }
            }
            Less => {
                let el_mid = arr[mid];
                while arr[mid] == el_mid {
                    mid += 1;
                }
            }
            Greater => {
                let el_end = arr[end];
                while arr[end] == el_end {
                    end -= 1;
                }
            }
        };
    }
    println!("Set of 3 elements whose sum is zero:- {:?}", ans);
}
