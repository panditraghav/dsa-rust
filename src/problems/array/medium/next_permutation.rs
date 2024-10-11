use std::fs::File;
use std::io::BufReader;

use crate::utils::input::get_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    get_vector_from_buf_reader::<i32>(&mut file_reader)
}

/// First we will find an element from starting from end who is smaller then the next element
/// because by replacing it with element just greater then it we can get the next permutation.
/// We will search from last because we want the nearest next permutation, number that is just
/// greater then given number.
///
/// Once this index is found, replace this index with the next greater index
/// to get the next biggest number we then sort the elements after this index
/// to make the section after this index the smallest
///
/// # Example
///
/// **Input**: 1 3 2
/// the breakpoint index will be 0 since 1 < 3 and 3 !< 2
/// so we swap 1 and 2 since 2 is the next greater number
/// we get array: 2 3 1
/// But this is not the next perumation for that we will sort the
/// elements after index 0
/// **Output**: 2 1 3
pub fn optimal() {
    let mut arr = get_input();
    println!("Input array is: {:?}", arr);
    let n = arr.len();
    let mut break_point: Option<usize> = None;

    for i in (0..(n - 1)).rev() {
        println!("{i}");
        if arr[i] < arr[i + 1] {
            break_point = Some(i);
            break;
        }
    }

    if let Some(bp) = break_point {
        // Replacing the breakpoint element index with element
        // which is just greater then it
        let mut next_greatest: usize = bp + 1;
        for i in (bp + 2)..n {
            if arr[i] < arr[next_greatest] {
                next_greatest = i;
            }
        }
        arr.swap(bp, next_greatest);

        let arr_slice_after_breakpoint = &mut arr[(bp + 1)..n];
        arr_slice_after_breakpoint.sort();

        println!("The next permuation is: {:?}", arr);
    } else {
        // Then this is the last permutation and we can then print the first permuation
        // by sorting the entire array
        println!("This is the last permutation!");
        arr.sort();
        println!("The next permuation (first one) is: {:?}", arr);
    }
}
