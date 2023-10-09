use std::env::var;
use std::io;
use std::process::Command;

use rand::Rng;

fn main() {
    let mut matrix = create_zero_matrix(10, 50);

    matrix[1][1] = 1;
    matrix[1][2] = 1;
    matrix[1][3] = 1;

    matrix[1][10] = 1;
    matrix[3][10] = 1;
    matrix[2][9] = 1;
    matrix[2][11] = 1;

    matrix[1][21] = 1;
    matrix[2][23] = 1;
    matrix[3][21] = 1;
    matrix[3][22] = 1;
    matrix[3][23] = 1;

    print_matrix(&matrix);

    let exit_char = String::from("x");

    loop {
        let s = &mut String::new();
        io::stdin().read_line(s).unwrap();

        if s.trim().eq(&exit_char) {
            return;
        }

        matrix = update_matrix(&matrix);

        print_matrix(&matrix);
    };
}

fn create_zero_matrix(row_number: usize, col_numer: usize) -> Vec<Vec<i32>> {
    vec![vec![0; col_numer]; row_number]
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for value in row {
            if *value == 0 {
                print!("░ ");
            } else {
                print!("█ ");
            }
        }
        println!();
    }
}

fn print_matrix_values(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for value in row {
            print!("{} ", value);
        }
        println!();
    }
}

fn create_random_matrix(row_number: usize, col_numer: usize) -> Vec<Vec<i32>> {
    let mut rng = rand::thread_rng();

    let mut matrix = Vec::with_capacity(row_number);
    for _ in 0..row_number {
        let mut row = Vec::with_capacity(col_numer);
        for _ in 0..col_numer {
            let value = if rng.gen_ratio(1, 10) { 1 } else { 0 };
            row.push(value);
        }
        matrix.push(row);
    }

    matrix
}

fn get_neighbor(row: usize, col: usize, matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut out = vec![vec![0; 3]; 3];

    let row_number = matrix.len() as usize;
    let col_number = matrix[0].len() as usize;

    for row_index in 0..3 {
        for col_index in 0..3 {
            let v_x: usize = (row_number + row + row_index as usize - 1) % row_number;
            let v_y: usize = (col_number + col + col_index as usize - 1) % col_number;

            out[row_index as usize][col_index as usize] = matrix[v_x][v_y];
        }
    }

    out
}

fn increment_neighbor(row: usize, col: usize, matrix: &mut Vec<Vec<i32>>) {
    let row_number = matrix.len() as usize;
    let col_number = matrix[0].len() as usize;

    for row_index in 0..3 {
        for col_index in 0..3 {
            let v_x: usize = (row_number + row + row_index as usize - 1) % row_number;
            let v_y: usize = (col_number + col + col_index as usize - 1) % col_number;
            // if v_x != row && v_y != col {
                matrix[v_x][v_y] += 1;
            // }
        }
    }
}

fn update_matrix(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let row_number = matrix.len() as usize;
    let col_number = matrix[0].len() as usize;
    let mut out = vec![vec![0; col_number]; row_number];
    let mut counter = vec![vec![0; col_number]; row_number];

    for i in 0..row_number {
        for j in 0..col_number {
            if matrix[i][j] == 1 {
                increment_neighbor(i, j, &mut counter)
            }
        }
    }

    print_matrix_values(&counter);

    for i in 0..row_number {
        for j in 0..col_number {
            if matrix[i][j] == 1 {
                if counter[i][j] - 1  < 2 {
                    out[i][j] = 0;
                } else if counter[i][j] - 1 > 3 {
                    out[i][j] = 0;
                } else {
                    out[i][j] = 1;
                }
            } else {
                if counter[i][j] == 3 {
                    out[i][j] = 1;
                } else {
                    out[i][j] = 0;
                }
            }
        }
    }

    out
}