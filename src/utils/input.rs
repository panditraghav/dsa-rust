use std::{io::stdin, str::FromStr};

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
