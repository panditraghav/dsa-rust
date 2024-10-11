#![allow(dead_code)]
use std::fmt::Debug;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::str::FromStr;

pub fn get_input_vector(size: usize) -> Vec<i32> {
    let mut arr: Vec<i32> = vec![0; size];
    println!("Enter {} elements for array (separated by spaces):-", size);

    let mut num_str = String::new();
    stdin().read_line(&mut num_str).unwrap();

    let split_arr: Vec<&str> = num_str.trim().split(" ").collect();
    if split_arr.len() != size {
        panic!(
            "The size of input array {in_size} is not equal to {size}",
            in_size = split_arr.len(),
            size = size
        );
    }

    for i in 0..size {
        let num_int = split_arr[i].trim().parse::<i32>().unwrap();
        arr[i] = num_int;
    }
    arr
}

pub fn get_num_input<T>() -> T
where
    T: FromStr,
{
    let mut num_str = String::new();
    stdin().read_line(&mut num_str).unwrap();
    let num_int = num_str.trim().parse::<T>();

    match num_int {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Some error occured while parsing string");
            panic!();
        }
    }
}

pub fn get_vector_from_buf_reader<T>(file_reader: &mut BufReader<File>) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut vec_str = String::new();
    file_reader.read_line(&mut vec_str).unwrap();

    let split_arr: Vec<&str> = vec_str.trim().split(" ").collect();

    let mut arr: Vec<T> = Vec::new();

    for item in split_arr {
        let num_int = (*item).trim().parse::<T>().unwrap();
        arr.push(num_int);
    }
    arr
}
pub fn get_num_from_reader<T>(file_reader: &mut BufReader<File>) -> T
where
    T: FromStr,
{
    let mut num_str = String::new();
    file_reader.read_line(&mut num_str).unwrap();
    let num_int = num_str.trim().parse::<T>();

    match num_int {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Some error occured while parsing string");
            panic!();
        }
    }
}
