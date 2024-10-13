use crate::utils::input::input_matrix_from_buf_reader;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

pub fn take_input() -> Vec<Vec<i32>> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_matrix_from_buf_reader::<i32>(&mut file_reader)
}

/// Just iterating thrugh the matrix twice, first time to get which rows and cols
/// are to be zeroed out and second time to zero them
/// **Time Complexity**: O(2*N*M)
/// There just one better approach which reduces space Complexity.
/// In that the first row and first column of matrix itself is used to keep track
/// of which row and col to be made zero, but that is not implementable in rust since
/// we cannot take a mutable reference to a vector while a immutable reference is in scope.
pub fn brute() {
    let mut mat = take_input();
    println!("The input matrix is: {:?}", mat);
    let mut zero_rows: HashSet<usize> = HashSet::new();
    let mut zero_cols: HashSet<usize> = HashSet::new();

    for (row_i, row) in mat.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            if *cell == 0 {
                zero_rows.insert(row_i);
                zero_cols.insert(col_i);
            }
        }
    }

    for zr in &zero_rows {
        for cell in &mut mat[*zr] {
            *cell = 0;
        }
    }
    for cz in &zero_cols {
        for col in &mut mat {
            col[*cz] = 0;
        }
    }

    println!("The new matrix is: {:?}", mat);
}
