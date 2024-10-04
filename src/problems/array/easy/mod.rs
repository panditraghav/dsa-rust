/// Given an integer N and an array of size N-1 containing N-1 numbers between 1 to N.
/// Find the number(between 1 to N), that is not present in the given array.
pub mod missing_element_in_array {
    use crate::utils::input::get_input_vector;
    use std::io::stdin;

    fn take_input() -> (Vec<i32>, u8) {
        println!("Enter number of elements: ");
        let mut num_str = String::new();
        stdin().read_line(&mut num_str).unwrap();
        let num_int = num_str.trim().parse::<u8>().unwrap();

        (get_input_vector(u32::from(num_int - 1)), num_int)
    }

    pub fn brute() {
        println!("missing_element_in_array: brute");
        let (arr, size) = take_input();
        println!("Array is:- {arr:?}");

        for i in 1..(i32::from(size)) {
            let mut found: bool = false;
            for j in arr.iter() {
                if *j == i {
                    found = true;
                }
            }
            if !found {
                println!("The missing element is: {i}");
                break;
            }
        }
    }
}
