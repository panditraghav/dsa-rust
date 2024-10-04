use std::io::stdin;

pub fn get_input_vector(size: u32) -> Vec<i32> {
    let mut arr: Vec<i32> = Vec::new();
    println!("Enter {} elements for array:-", size);
    for i in 1..(size + 1) {
        println!("Enter element {}", i);
        let mut num_str = String::new();
        stdin().read_line(&mut num_str).unwrap();
        let num_int = num_str.trim().parse::<i32>().unwrap();
        arr.push(num_int);
    }
    arr
}
