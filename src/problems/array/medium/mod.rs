#![allow(dead_code)]

/// Given an array of integers arr[] and an integer target
/// Return indices of the two numbers such that their sum is equal to the target.
/// Otherwise, we will return {-1, -1}.
pub mod two_sum;

/// Given an array consisting of only 0s, 1s, and 2s.
/// Write a program to in-place sort the array without using inbuilt sort functions.
/// ( Expected: Single pass-O(N) and constant space)
pub mod sort_zero_one_two_array;

/// Given an array of N integers, write a program to return an element
/// that occurs more than N/2 times in the given array.
/// You may consider that such an element always exists in the array
pub mod majority_element;

/// Given an integer array arr, find the contiguous subarray (containing at least one number) which
/// has the largest sum and returns its sum and prints the subarray.
pub mod max_subarray_sum;

/// You are given an array of prices where prices[i] is the price of a given stock on an ith day.
/// You want to maximize your profit by choosing a single day to buy one stock and choosing a
/// different day in the future to sell that stock.
/// Return the maximum profit you can achieve from this transaction.
/// If you cannot achieve any profit, return 0.
pub mod stock_buy_sell;

/// There’s an array ‘A’ of size ‘N’ with an equal number of positive and negative elements.
/// Without altering the relative order of positive and negative elements,
/// you must return an array of alternately positive and negative values.
pub mod rearrange_elements_by_sign;

/// Given an array Arr[] of integers, rearrange the numbers of the given array
/// into the lexicographically next greater permutation of numbers.
/// # Example 1 :
/// Input format:
///  Arr[] = {1,3,2}
/// Output
/// : Arr[] = {2,1,3}
/// ## Explanation:
/// All permutations of {1,2,3} are {{1,2,3} , {1,3,2}, {2,13} , {2,3,1} , {3,1,2} , {3,2,1}}.
/// So, the next permutation just after {1,3,2} is {2,1,3}.
pub mod next_permutation;

/// Given an array, print all the elements which are leaders.
/// A Leader is an element that is greater than all of the elements
/// on its right side in the array.
pub mod leaders_in_array;

/// You are given an array of ‘N’ integers. You need to find the length
/// of the longest sequence which contains the consecutive elements.
///
/// # Example 1:
/// **Input**:
///  {100, 200, 1, 3, 2, 4}
/// **Output**:
///  4
/// **Explanation**:
///  The longest consecutive subsequence is 1, 2, 3, and 4.
///
/// # Example 2:
/// **Input**:
///  {3, 8, 5, 7, 6}
/// **Output**:
///  4
/// **Explanation**:
///  The longest consecutive subsequence is 5, 6, 7, and 8.
pub mod longest_consecutive_sequence;
