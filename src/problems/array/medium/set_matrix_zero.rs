use crate::utils::input::input_matrix_from_buf_reader;
use std::fs::File;
use std::io::BufReader;

pub fn take_input() {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    let mat = input_matrix_from_buf_reader::<i32>(&mut file_reader);
    println!("Matrix is: {:?}", mat);
}

pub fn brute() {
    take_input();
}
