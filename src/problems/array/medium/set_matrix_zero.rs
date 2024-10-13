use crate::utils::input::input_matrix_from_buf_reader;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

pub fn take_input() -> Vec<Vec<i32>> {
    let file = File::open("./inputs.txt").unwrap();
    let mut file_reader = BufReader::new(file);

    input_matrix_from_buf_reader::<i32>(&mut file_reader)
}

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
