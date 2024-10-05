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

        (get_input_vector(num_int - 1), num_int)
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

    /// Using the sum of first n natural number formula to find `sum_of_first_n_num`
    /// and subtracting `sum_of_arr` from it to find the missing number.
    /// The difference will be the missing number!
    pub fn optimal() {
        println!("missing_element_in_array: optimal");

        let (arr, size) = take_input();
        println!("Array is:- {arr:?}");

        let sum_of_first_n_num = size * (size + 1) / 2;
        let mut sum_of_arr = 0;

        for i in arr {
            sum_of_arr += i;
        }

        println!(
            "The missing number is {}",
            i32::from(sum_of_first_n_num) - sum_of_arr
        );
    }

    /// Using xor operation
    /// n ^ n = 0;
    /// n ^ 0 = n;
    /// by this logic we first calculate xor of first n natural numbers (1 ^ 2 ^ 3 ^ 4 ^ 5)
    /// And then calculate xor of all numbers of array (1 ^ 2 ^ 4 ^ 5)
    /// Then we xor these two values, all the same numbers will become 0
    /// and the missing number ^ 0 will give us missing number;
    pub fn optimal_xor() {
        println!("missing_element_in_array: optimal_xor");

        let (arr, size) = take_input();
        println!("Array is:- {arr:?}");

        let mut xor1 = 0;
        let mut xor2: u32 = 0;

        for i in 0..(u32::from(size) - 1) {
            xor1 ^= i + 1;
            let v = arr.get(usize::try_from(i).unwrap()).unwrap().unsigned_abs();
            xor2 ^= v;
        }
        xor1 ^= u32::from(size);

        println!("The missing number is {}", xor1 ^ xor2);
    }
}

pub mod find_max_consecutive_ones {
    use crate::utils::input::get_input_vector;

    pub fn optimal() {
        println!("find_max_consecutive_ones: optimal");
        let arr = get_input_vector(8);

        let mut counter = 0;
        let mut max = 0;
        for i in arr {
            if i == 1 {
                counter += 1;
                if counter > max {
                    max = counter
                }
            } else {
                counter = 0;
            }
        }

        println!("Maximum consecutive ones are: {}", max);
    }
}
