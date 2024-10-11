use std::collections::HashMap;

use crate::utils::input::{get_input_vector, get_num_input};
use std::cmp::Ordering::{Equal, Greater, Less};

fn take_input() -> (Vec<i32>, i32) {
    println!("Enter number of elements: ");
    let num_element: usize = get_num_input();

    let arr = get_input_vector(num_element);

    println!("Enter sum: ");
    let sum: i32 = get_num_input();

    (arr, sum)
}

#[derive(Debug)]
struct Indexes {
    i: usize,
    j: usize,
}

/// Simply using two for loops to calculate sum
pub fn brute() {
    println!("Two sum problem: brute");
    let (arr, sum) = take_input();
    let n = arr.len();

    let mut indexes: Option<Indexes> = None;
    for i in 0..n {
        let e1 = arr[i];

        if indexes.is_some() {
            break;
        }
        if i == n - 1 {
            break;
        }

        for j in (i + 1)..n {
            if e1 + arr[j] == sum {
                indexes = Some(Indexes { i, j });
                break;
            }
        }
    }

    match indexes {
        Some(ind) => {
            println!("Sum found at index: {:?}", ind);
        }
        None => {
            println!("Sum not found");
        }
    };
}

/// Using hashmap
pub fn better() {
    let (arr, sum) = take_input();
    let n = arr.len();
    let mut index_map: HashMap<i32, usize> = HashMap::new();

    let mut indexes: Option<Indexes> = None;

    for i in 0..n {
        if indexes.is_some() {
            break;
        }
        let current_item = arr[i];

        match index_map.get(&(sum - current_item)) {
            Some(index) => {
                indexes = Some(Indexes { i, j: *index });
            }
            None => {
                index_map.insert(current_item, i);
            }
        }
    }

    match indexes {
        Some(ind) => {
            println!("Sum found at index: {:?}", ind);
        }
        None => {
            println!("Sum not found");
        }
    };
}

/// First sorting the array and then using two pointer approach.
/// This will reduce the space complexity since not storing a HashMap,
/// And the complexity will be nlogn same as using HashMap
/// But this will give wrong index as the array is sorted
pub fn optimal() {
    let (mut arr, sum) = take_input();
    arr.sort_unstable();

    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;

    let mut indexes: Option<Indexes> = None;

    while left < right {
        let current_sum = arr[left] + arr[right];

        match current_sum.cmp(&sum) {
            Equal => {
                indexes = Some(Indexes { i: left, j: right });
            }
            Greater => {
                right -= 1;
            }
            Less => {
                left += 1;
            }
        }
        if indexes.is_some() {
            break;
        }
    }

    match indexes {
        Some(ind) => {
            println!("Sum found at index: {:?}", ind);
        }
        None => {
            println!("Sum not found");
        }
    };
}
