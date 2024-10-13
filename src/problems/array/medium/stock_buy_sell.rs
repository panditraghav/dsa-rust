use std::fs::File;
use std::io::BufReader;

use crate::utils::input::input_vector_from_buf_reader;

fn get_input() -> Vec<i32> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_vector_from_buf_reader::<i32>(&mut file_reader).unwrap()
}

#[derive(Debug)]
struct ProfitDays {
    buy: usize,
    sell: usize,
}

/// For every element of array checking all the next elements for largest price difference
pub fn brute() {
    let arr = get_input();
    let n = arr.len();
    let mut maximum_profit = 0;
    let mut max_profit_days = ProfitDays { buy: 1, sell: 2 };

    for i in 0..n {
        for j in (i + 1)..n {
            let current_profit = arr[j] - arr[i];
            if current_profit > maximum_profit {
                maximum_profit = current_profit;
                max_profit_days.buy = i + 1;
                max_profit_days.sell = j + 1;
            }
        }
    }

    println!(
        "Maximum profit is: {}\nThe days are: {:?}",
        maximum_profit, max_profit_days
    );
}

/// We will linearly travel the array. We can maintain a minimum from
/// the start of the array and compare it with every element of the array,
/// if it is greater than the minimum then take the difference and maintain
/// it in max, otherwise update the minimum
pub fn optimal() {
    let arr = get_input();
    let n = arr.len();
    let mut maximum_profit = 0;
    let mut min_price = arr[0];
    let mut max_profit_days = ProfitDays { buy: 1, sell: 2 };

    for (i, el_i) in arr[1..n].iter().enumerate() {
        if *el_i < min_price {
            min_price = *el_i;
            max_profit_days.buy = i + 1;
        } else if *el_i - min_price > maximum_profit {
            maximum_profit = *el_i - min_price;
            max_profit_days.sell = i + 1;
        }
    }

    println!(
        "Maximum profit is: {}\nThe days are: {:?}",
        maximum_profit, max_profit_days
    );
}
