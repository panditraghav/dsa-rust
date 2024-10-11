use std::collections::HashMap;

use crate::utils::input::{get_input_vector, get_num_input};

fn take_input() -> Vec<i32> {
    println!("Enter number of elements: ");
    let num_element: usize = get_num_input();
    println!("{} / 2 = {}", num_element, num_element / 2);

    get_input_vector(num_element)
}

/// Using two for loops to count each element
/// Complexity : O(n^2)
pub fn brute() {
    let arr = take_input();
    let n = arr.len();

    for i in 0..n {
        let current = arr[i];
        let mut current_count: usize = 0;
        for j in &arr {
            if *j == current {
                current_count += 1;
            }
        }
        if current_count > n / 2 {
            println!("Found the majority_element: {}", current);
            break;
        }
    }
}

/// Using HashMap this will lead to one iteration of array
/// to insert elements into map and other iteration on the
/// hashmap
/// Complexity : O(nlogn) + O(n)
pub fn better() {
    let arr = take_input();
    let mut element_count_map: HashMap<&i32, usize> = HashMap::new();

    for i in &arr {
        if let Some(old) = element_count_map.get_mut(i) {
            *old += 1;
        } else {
            element_count_map.insert(i, 1);
        }
    }

    for (k, v) in element_count_map {
        if v > arr.len() / 2 {
            println!("Found the majority_element: {}", k);
            break;
        }
    }
}

/// # Using Moore’s Voting Algorithm
/// Here we are taking two variables element and count
/// We see if some element is majority or not in a subarray
/// if count becomes 0 then it is not majority in that subarray
/// and if at the last subarray count is not 0 then that means that
/// there is some element which is majority in that subarray,
/// and will be in the whole if majority element is there
pub fn optimal() {
    let arr = take_input();
    let mut el = arr[0];
    let mut count = 1;

    for i in &arr {
        if count == 0 {
            el = *i;
        }
        if *i == el {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count == 0 {
        println!("No majority element found!");
        return;
    }
    count = 0;
    for i in &arr {
        if el == *i {
            count += 1;
        }
    }
    if count > arr.len() / 2 {
        println!("Found the majority_element: {}", el);
    } else {
        println!("No majority element found!");
    }
}
