use std::fs::File;
use std::io::BufReader;

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_vector_from_buf_reader::<i32>(&mut file_reader)
}

/// We create two arrays one containing all positives and other all negatives
/// And then traverse these arrays and place elements at alternate positions
pub fn brute() {
    let mut arr = get_input();
    println!("Input array is: {:?}", arr);
    let n = arr.len();
    let mut positives: Vec<i32> = Vec::with_capacity(n / 2);
    let mut negatives: Vec<i32> = Vec::with_capacity(n / 2);

    for el in &arr {
        if *el >= 0 {
            positives.push(*el);
        } else {
            negatives.push(*el);
        }
    }
    for i in 0..positives.len() {
        arr[i * 2] = positives[i];
    }
    for i in 0..negatives.len() {
        arr[i * 2 + 1] = negatives[i];
    }

    println!("The rearranged array is: {:?}", arr);
}

/// We can do this in one pass by creating new array and whenever we find
/// a positive number we assign it to positive index and other to negative index
/// Here instead of mutating the same array we can return the answer from function
/// or run another loop to mutate the same array but then the time complexity will be same
pub fn optimal() {
    let arr = get_input();
    println!("Input array is: {:?}", arr);
    let n = arr.len();
    let mut ans: Vec<i32> = vec![0; n];
    let mut positive_i = 0;
    let mut negative_i = 1;

    for el in &arr {
        if *el >= 0 {
            ans[positive_i] = *el;
            positive_i += 2;
        } else {
            ans[negative_i] = *el;
            negative_i += 2;
        }
    }

    println!("The rearranged array is: {:?}", ans);
}
