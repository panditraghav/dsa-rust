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
