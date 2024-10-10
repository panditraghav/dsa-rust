/// Given an array of integers arr[] and an integer target
/// Return indices of the two numbers such that their sum is equal to the target.
/// Otherwise, we will return {-1, -1}.
pub mod two_sum {
    use std::collections::HashMap;

    use crate::utils::input::{get_input_vector, get_num_input};
    use std::cmp::Ordering::{Equal, Greater, Less};

    fn take_input() -> (Vec<i32>, i32) {
        println!("Enter number of elements: ");
        let num_element: usize = get_num_input();

        let arr = get_input_vector(num_element);

        println!("Enter sum: ");
        let sum: i32 = get_num_input();

        (arr, sum)
    }

    #[derive(Debug)]
    struct Indexes {
        i: usize,
        j: usize,
    }

    /// Simply using two for loops to calculate sum
    pub fn brute() {
        println!("Two sum problem: brute");
        let (arr, sum) = take_input();
        let n = arr.len();

        let mut indexes: Option<Indexes> = None;
        for i in 0..n {
            let e1 = arr[i];

            if indexes.is_some() {
                break;
            }
            if i == n - 1 {
                break;
            }

            for j in (i + 1)..n {
                if e1 + arr[j] == sum {
                    indexes = Some(Indexes { i, j });
                    break;
                }
            }
        }

        match indexes {
            Some(ind) => {
                println!("Sum found at index: {:?}", ind);
            }
            None => {
                println!("Sum not found");
            }
        };
    }

    /// Using hashmap
    pub fn better() {
        let (arr, sum) = take_input();
        let n = arr.len();
        let mut index_map: HashMap<i32, usize> = HashMap::new();

        let mut indexes: Option<Indexes> = None;

        for i in 0..n {
            if indexes.is_some() {
                break;
            }
            let current_item = arr[i];

            match index_map.get(&(sum - current_item)) {
                Some(index) => {
                    indexes = Some(Indexes { i, j: *index });
                }
                None => {
                    index_map.insert(current_item, i);
                }
            }
        }

        match indexes {
            Some(ind) => {
                println!("Sum found at index: {:?}", ind);
            }
            None => {
                println!("Sum not found");
            }
        };
    }

    /// First sorting the array and then using two pointer approach.
    /// This will reduce the space complexity since not storing a HashMap,
    /// And the complexity will be nlogn same as using HashMap
    /// But this will give wrong index as the array is sorted
    pub fn optimal() {
        let (mut arr, sum) = take_input();
        arr.sort_unstable();

        let mut left: usize = 0;
        let mut right: usize = arr.len() - 1;

        let mut indexes: Option<Indexes> = None;

        while (left < right) {
            let current_sum = arr[left] + arr[right];

            match current_sum.cmp(&sum) {
                Equal => {
                    indexes = Some(Indexes { i: left, j: right });
                }
                Greater => {
                    right -= 1;
                }
                Less => {
                    left += 1;
                }
            }
            if indexes.is_some() {
                break;
            }
        }

        match indexes {
            Some(ind) => {
                println!("Sum found at index: {:?}", ind);
            }
            None => {
                println!("Sum not found");
            }
        };
    }
}

/// Given an array consisting of only 0s, 1s, and 2s.
/// Write a program to in-place sort the array without using inbuilt sort functions.
/// ( Expected: Single pass-O(N) and constant space)
pub mod sort_array_consisting_zero_one_two {
    use crate::utils::input::{get_input_vector, get_num_input};

    fn take_input() -> Vec<i32> {
        println!("Enter number of elements: ");
        let num_element: usize = get_num_input();

        get_input_vector(num_element)
    }

    /// Sorting array, complexity O(nlogn)
    pub fn brute() {
        let mut arr = take_input();

        arr.sort();
        println!("Sorted array is: {:?}", arr);
    }

    /// Getting count of 0s, 1s and 2s in one iteration
    /// And replacing the elements on second iteration using the count
    pub fn better() {
        let mut arr = take_input();

        let mut zero_count = 0;
        let mut one_count = 0;
        let mut two_count = 0;

        for i in arr.iter() {
            match *i {
                0 => {
                    zero_count += 1;
                }
                1 => {
                    one_count += 1;
                }
                2 => {
                    two_count += 1;
                }
                _ => {}
            }
        }

        let mut i = 0;
        for _ in 0..zero_count {
            arr[i] = 0;
            i += 1;
        }
        for _ in 0..one_count {
            arr[i] = 1;
            i += 1;
        }
        for _ in 0..two_count {
            arr[i] = 2;
            i += 1;
        }
        println!("Sorted array is: {:?}", arr);
    }

    /// Using dutch national flag algorithm
    /// This algorithm uses 3 pointers: low, mid, high
    /// It follows 4 rules
    /// 1. {0..(low -1)} => should be 0 (Extreme left)
    /// 2. {low...(mid -1)} => should be 1 (Mid portion)
    /// 3. {mid...high} => Any random numbers
    /// 4. {(high+1)...(n-1)} => should be 2 (End portion)
    ///
    /// So the hypothetical array will be something like this:-
    /// {0..(low-1),low...(mid -1),mid...high,(high+1)...(n-1)}
    /// ----0------|------1------|random_num|-------2--------
    pub fn optimal() {
        let mut arr = take_input();

        let mut low: usize = 0;
        let mut mid: usize = 0;
        let mut high: usize = arr.len() - 1;

        while mid <= high {
            match arr[mid] {
                0 => {
                    arr.swap(mid, low);

                    low += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                2 => {
                    arr.swap(mid, high);

                    high -= 1;
                }
                _ => (),
            }
        }

        println!("The sorted array is: {:?}", arr);
    }
}

/// Given an array of N integers, write a program to return an element
/// that occurs more than N/2 times in the given array.
/// You may consider that such an element always exists in the array
pub mod majority_element {
    use std::collections::HashMap;

    use crate::utils::input::{get_input_vector, get_num_input};

    fn take_input() -> Vec<i32> {
        println!("Enter number of elements: ");
        let num_element: usize = get_num_input();
        println!("{} / 2 = {}", num_element, num_element / 2);

        get_input_vector(num_element)
    }

    /// Using two for loops to count each element
    /// Complexity : O(n^2)
    pub fn brute() {
        let arr = take_input();
        let n = arr.len();

        for i in 0..n {
            let current = arr[i];
            let mut current_count: usize = 0;
            for j in &arr {
                if *j == current {
                    current_count += 1;
                }
            }
            if current_count > n / 2 {
                println!("Found the majority_element: {}", current);
                break;
            }
        }
    }

    /// Using HashMap this will lead to one iteration of array
    /// to insert elements into map and other iteration on the
    /// hashmap
    /// Complexity : O(nlogn) + O(n)
    pub fn better() {
        let arr = take_input();
        let mut element_count_map: HashMap<&i32, usize> = HashMap::new();

        for i in &arr {
            if let Some(old) = element_count_map.get_mut(i) {
                *old += 1;
            } else {
                element_count_map.insert(i, 1);
            }
        }

        for (k, v) in element_count_map {
            if v > arr.len() / 2 {
                println!("Found the majority_element: {}", k);
                break;
            }
        }
    }
}
