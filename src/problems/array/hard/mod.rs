/// Given an array containing both positive and negative integers,
/// we have to find the length of the longest subarray with the
/// sum of all elements equal to zero.
/// # Example 1:
/// **Input**
/// N = 6, array[] = {9, -3, 3, -1, 6, -5}
/// **Result**
/// 5 (-3, 3, -1, 6, -5)
pub mod largest_subarray_with_zero_sum;

/// Given an array of N integers, your task is to find unique triplets
/// that add up to give a sum of zero. In short, you need to return an
/// array of all the unique triplets [arr[a], arr[b], arr[c]] such
/// that i!=j, j!=k, k!=i, and their sum is equal to zero.
/// # Example 1:
///
/// **Input**:
///  nums = {-1,0,1,2,-1,-4}
/// **Output**:
///  {{-1,-1,2},{-1,0,1}}
pub mod three_sum;

/// Given an array of integers A and an integer B.
/// Find the total number of subarrays having bitwise XOR of all elements equal to k.
///
/// # Example 1:
/// **Input**:
///  A = {4, 2, 2, 6, 4} , k = 6
/// **Result**:
///  4
/// **Explanation**:
///  The subarrays having XOR of their elements as 6 are
///  {4, 2}, {4, 2, 2, 6, 4}, {2, 2, 6}, {6}
pub mod num_of_subarray_with_xor_k;

/// Given an array of intervals, merge all the overlapping
/// intervals and return an array of non-overlapping intervals.
///
/// # Example 1:
/// **Input**:
///  intervals={{1,3},{2,6},{8,10},{15,18}}
/// **Output**:
///  {{1,6},{8,10},{15,18}}
///
/// **Explanation**:
///  Since intervals {1,3} and {2,6} are overlapping we can merge them to form {1,6}
///  intervals.
pub mod merge_overlapping_subintervals;

/// # Problem statement:
/// Given two sorted arrays arr1[] and arr2[] of sizes n and m
/// in non-decreasing order. Merge them in sorted order.
/// Modify arr1 so that it contains the first N elements and modify arr2
/// so that it contains the last M elements.
///
/// # Example 1:
///
/// **Input**:
/// n = 4, arr1{} = {1 4 8 10}
/// m = 5, arr2{} = {2 3 9}
///
/// **Output**:
/// arr1{} = {1 2 3 4}
/// arr2{} = {8 9 10}
pub mod merge_two_sorted_arrays;

/// Problem Statement: You are given a read-only array of N integers
/// with values also in the range [1, N] both inclusive.
/// Each integer appears exactly once except A which appears twice and B which is missing.
/// The task is to find the repeating and missing numbers A and B where A repeats twice and B is missing.
///
/// # Example 1:
///
/// **Input**
/// :  array = {3,1,2,5,3}
/// **Result**
/// : {3,4}
/// **Explanation**
/// : A = 3 , B = 4
/// Since 3 is appearing twice and 4 is missing
pub mod find_repeating_and_missing_numbers;

/// # Problem Statement:
/// Given an array of N integers, count the inversion of the array (using merge-sort).
/// What is an inversion of an array? Definition: for all i & j < size of array,
/// if i < j then you have to find pair (A{i},A{j}) such that A{j} < A{i}.
///
/// # Example 1:
/// **Input**
/// : N = 5, array = {5,4,3,2,1}
/// **Result**
/// : 10
/// **Explanation**
/// : we have a reverse sorted array and we will get the maximum inversions
/// as for i < j we will always find a pair such that A[j] < A[i].
/// Example: 5 has index 0 and 3 has index 2 now (5,3) pair is inversion
/// as 0 < 2 and 5 > 3 which will satisfy out conditions and for reverse sorted
/// array we will get maximum inversions and that is (n)*(n-1) / 2.
/// For above given array there is 4 + 3 + 2 + 1 = 10 inversions.
///
/// # Example 2:
///
/// **Input**
/// : N = 5, array = {5,3,2,1,4}
/// **Result**
/// : 7
/// **Explanation**
/// : There are 7 pairs (5,1), (5,3), (5,2), (5,4),(3,2), (3,1), (2,1)
/// and we have left 2 pairs (2,4) and (1,4) as both are not satisfy our condition.
pub mod count_inversion;
