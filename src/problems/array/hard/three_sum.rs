use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap()
}

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
