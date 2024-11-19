mod problems;
mod utils;
use problems::array::hard::merge_two_sorted_arrays as current_problem;

fn main() {
    current_problem::with_extra_space();
    current_problem::without_extra_space();
}
