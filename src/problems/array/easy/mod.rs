/// Given an integer N and an array of size N-1 containing N-1 numbers between 1 to N.
/// Find the number(between 1 to N), that is not present in the given array.
pub mod missing_element_in_array {
    use crate::utils::input::get_input_vector;
    use std::{collections::HashMap, io::stdin};

    fn take_input() -> (Vec<i32>, u8) {
        println!("Enter number of elements: ");
        let mut num_str = String::new();
        stdin().read_line(&mut num_str).unwrap();
        let num_int = num_str.trim().parse::<u8>().unwrap();

        (get_input_vector(u32::from(num_int - 1)), num_int)
    }
    /// Using nested loop
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

    // Usin hash map
    pub fn better() {
        println!("missing_element_in_array: better");

        let (arr, size) = take_input();
        println!("Array is:- {arr:?}");

        let mut map: HashMap<i32, u8> = HashMap::new();

        for i in arr {
            map.insert(i, 1);
        }

        let mut not_found: u8 = 0;
        for i in 1..size {
            let val = map.get(&i32::from(i));
            match val {
                None => {
                    not_found = i;
                    break;
                }
                Some(_) => (),
            };
        }
        println!("The missing number is {not_found}");
    }
}
