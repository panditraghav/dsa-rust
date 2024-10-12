use std::fs::File;
use std::io::BufReader;

use crate::utils::input::get_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    get_vector_from_buf_reader::<i32>(&mut file_reader)
}

/// Using 2 for loops comparing every element
/// with elements after it to see if it is leader or not
pub fn brute() {
    println!("leaders_in_array: brute");
    let arr = get_input();
    let n = arr.len();
    println!("The input array is: {:?}", arr);
    let mut leader_arr: Vec<i32> = Vec::new();

    for i in (0..n).rev() {
        let el = arr[i];
        let mut is_leader = true;
        for j in &arr[(i + 1)..n] {
            if el < *j {
                is_leader = false;
            }
        }
        if is_leader {
            leader_arr.push(el);
        }
    }

    println!("The input array is: {:?}", arr);
    println!("The leaders of the input array are: {:?}", leader_arr);
}

/// Iterating the loop from end and keeping track of the largest
/// element we find, the largest element until that point and the next
/// largest element will always be the leader
pub fn optimal() {
    println!("leaders_in_array: optimal");
    let arr = get_input();
    let mut leader_arr: Vec<i32> = Vec::new();
    let mut largest_element: i32 = i32::MIN;

    for el in arr.iter().rev() {
        // The next largest element will always be bigger then all the elements
        // after that since the previous largest element which appears after
        // the current one is smaller then this one
        if *el >= largest_element {
            largest_element = *el;
            leader_arr.push(*el);
        }
    }
    println!("The input array is: {:?}", arr);
    println!("The leaders of the input array are: {:?}", leader_arr);
}
