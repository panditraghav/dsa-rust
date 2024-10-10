#![allow(dead_code)]
/// Given an integer N and an array of size N-1 containing N-1 numbers between 1 to N.
/// Find the number(between 1 to N), that is not present in the given array.
pub mod missing_element_in_array {
    use crate::utils::input::get_input_vector;
    use std::{collections::HashMap, io::stdin};

    fn take_input() -> (Vec<i32>, usize) {
        println!("Enter number of elements: ");
        let mut num_str = String::new();
        stdin().read_line(&mut num_str).unwrap();
        let num_int = num_str.trim().parse::<usize>().unwrap();

        (get_input_vector(num_int - 1), num_int)
    }
    /// Using nested loop
    pub fn brute() {
        println!("missing_element_in_array: brute");
        let (arr, size) = take_input();
        println!("Array is:- {arr:?}");

        for i in 1..(i32::try_from(size).unwrap()) {
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

        let mut not_found: usize = 0;
        for i in 1..size {
            let val = map.get(&i32::try_from(i).unwrap());
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
            i32::try_from(sum_of_first_n_num).unwrap() - sum_of_arr
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

        let mut xor1: usize = 0;
        let mut xor2: usize = 0;

        for i in 0..(size - 1) {
            xor1 ^= i + 1;
            let v = arr.get(usize::try_from(i).unwrap()).unwrap().unsigned_abs();
            xor2 ^= usize::try_from(v).unwrap();
        }
        xor1 ^= size;

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

/// Find the number that appears once given that
/// other number in this array appears twice
pub mod find_num_that_appears_once {
    use std::collections::HashMap;

    use crate::utils::input::get_input_vector;

    /// Using nested loop
    pub fn brute() {
        todo!(
            r#"Use nested loops to linear search every element and 
        see if there is an element for whome another element 
        was not found"#
        );
    }

    /// Using HashMap
    pub fn better() {
        println!("find_num_that_appears_once: better");
        let arr = get_input_vector(7);

        let mut count_map: HashMap<i32, u8> = HashMap::new();

        for i in arr {
            if let Some(v) = count_map.insert(i, 1) {
                *count_map.get_mut(&i).unwrap() += v;
            }
        }

        for (key, val) in count_map {
            if val == 1 {
                println!("The number that appears once is: {}", key);
                return;
            }
        }
    }

    /// Using xor operations
    pub fn optimal() {
        println!("find_num_that_appears_once: optimal");
        let arr = get_input_vector(7);

        let mut num_once = 0;
        for i in arr {
            num_once ^= i;
        }

        println!("The number that appeared once is {}", num_once);
    }
}

/// Given an array and a sum k, we need to print the length of the longest subarray that sums to k
pub mod longest_subarray_with_given_sum_positive {
    use std::{cmp::max_by, collections::HashMap, io::stdin};

    use crate::utils::input::get_input_vector;

    #[derive(Debug)]
    struct SubArray {
        left: usize,
        right: usize,
    }

    fn take_inputs() -> (Vec<i32>, i32) {
        println!("Enter size of array: ");
        let mut arr_size_str = String::new();
        stdin().read_line(&mut arr_size_str).unwrap();
        let arr_size = arr_size_str.trim().parse::<usize>().unwrap();

        let arr = get_input_vector(arr_size);

        println!("Enter sum: ");
        let mut input_num_str = String::new();
        stdin().read_line(&mut input_num_str).unwrap();
        let k = input_num_str.trim().parse::<i32>().unwrap();

        (arr, k)
    }

    /// Find sum of all sub arrays, and return the max
    pub fn brute() {
        println!("longest_subarray_with_given_sum: brute");

        let (arr, k) = take_inputs();

        let mut longest_subarray_len: usize = 0;
        let mut size_sub_arr: HashMap<usize, SubArray> = HashMap::new();

        for i in 0..(arr.len()) {
            let mut sum = 0;
            for j in i..(arr.len()) {
                sum += arr[j];
                if sum == k && j - i + 1 > longest_subarray_len {
                    longest_subarray_len = j - i + 1;
                    size_sub_arr.insert(longest_subarray_len, SubArray { left: i, right: j });
                }
                if sum > k {
                    break;
                }
            }
        }
        let sub_array = size_sub_arr.get(&longest_subarray_len).unwrap();
        println!("Longest subarray is of size {}", longest_subarray_len);
        println!("The subarray is: {:?}", sub_array);
    }

    /// Using hashmap to keep track of sum at all the indexes
    pub fn better() {
        println!("longest_subarray_with_given_sum: better");
        let (arr, k) = take_inputs();

        let mut longest_subarray_len: usize = 0;

        // storing the <sum_of_subarray_from_index_0_to_n, n>
        let mut len_map: HashMap<i32, usize> = HashMap::new();

        let mut temp_sum = 0;
        for i in 0..(arr.len()) {
            temp_sum += arr[i];

            len_map.insert(temp_sum, i);
            if temp_sum == k {
                longest_subarray_len =
                    max_by(longest_subarray_len, i + 1, |x: &usize, y: &usize| x.cmp(y));
            }

            let diff = temp_sum - k;

            // Since the hashmap stores the sum at any index then if there is any
            // key which is the difference between current_sum and k then sub array from
            // one index after that till i will have sum k
            if let Some(v) = len_map.get(&diff) {
                longest_subarray_len =
                    max_by(longest_subarray_len, i - v, |x: &usize, y: &usize| x.cmp(y));
            }
        }
        println!("Longest subarray is of size {}", longest_subarray_len);
    }

    /// Using two pointers
    pub fn optimal() {
        println!("longest_subarray_with_given_sum: optimal");
        let (arr, k) = take_inputs();

        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut sum = arr[0];
        let mut longest_subarray_len: usize = 1;
        let mut size_sub_arr: HashMap<usize, SubArray> = HashMap::new();
        let n = arr.len();

        while left < n - 1 {
            if left < right && sum > k {
                sum -= arr[left];
                left += 1;
                println!("left: {}, right: {}, sum: {}", left, right, sum);
                if sum == k {
                    longest_subarray_len =
                        max_by(longest_subarray_len, right - left + 1, |x, y| x.cmp(y));
                }
                continue;
            }

            if right < n - 1 {
                right += 1;
                sum += arr[right];
                if sum == k && right - left + 1 > longest_subarray_len {
                    longest_subarray_len = right - left + 1;
                    size_sub_arr.insert(longest_subarray_len, SubArray { left, right });
                }
            }
            println!("left: {}, right: {}, sum: {}", left, right, sum);
        }

        let sub_array = size_sub_arr.get(&longest_subarray_len).unwrap();
        println!("Longest subarray is of size {}", longest_subarray_len);
        println!("The subarray is: {:?}", sub_array);
    }
}
