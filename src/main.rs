mod problems;
mod utils;
use problems::array::hard::num_of_subarray_with_xor_k as current_problem;

fn main() {
    current_problem::brute();
    current_problem::optimal();
}
