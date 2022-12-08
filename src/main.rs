use array2d::Array2D;
use regex::Regex;
use std::fs;
use std::iter::Peekable;
use std::str::Lines;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut matrix = Array2D::filled_with(
        0,
        contents.lines().count(),
        contents.lines().nth(0).unwrap().len(),
    );


    for (row, line) in contents.lines().enumerate() {
        for (column, ch) in line.chars().enumerate() {
            matrix.set(row, column, ch.to_digit(10).unwrap());
        }
    }

    dbg!(matrix.clone());

    let mut vis_matrix = Array2D::filled_with(false, matrix.num_rows(), matrix.num_rows());
    for x in 1..vis_matrix.num_columns() - 1 {
        for y in 1..vis_matrix.num_rows() - 1 {
            let hight = matrix[(y, x)];
            let right_check = matrix.row_iter(y).skip(x + 1).fold(true, |mut acc, cur_hight| {
                if !acc {
                    return acc;
                }
                if *cur_hight >= hight {
                    acc = false;
                };
                acc
            });
            let mut left_check = matrix.row_iter(y).collect::<Vec<_>>();
            left_check.resize(x, &0);
            left_check.reverse();
            let left_check = left_check.iter().fold(true, |mut acc, cur_hight| {
                if !acc {
                    return acc;
                }
                if **cur_hight >= hight {
                    acc = false;
                };
                acc
            });

            let down_check = matrix
                .column_iter(x)
                .skip(y + 1)
                .fold(true, |mut acc, cur_hight| {
                    if !acc {
                        return acc;
                    }
                    if *cur_hight >= hight {
                        acc = false;
                    };
                    acc
                });
            let mut up_check = matrix.column_iter(x).collect::<Vec<_>>();
            up_check.resize(y, &0);
            up_check.reverse();
            let up_check = up_check.iter().fold(true, |mut acc, cur_hight| {
                if !acc {
                    return acc;
                }
                if **cur_hight >= hight {
                    acc = false;
                };
                acc
            });
            vis_matrix[(y, x)] = right_check || left_check || up_check || down_check;
        }
    }

    let outer_layer_visaable = matrix.num_columns() * 2 + (matrix.num_rows() - 2) * 2;

    let score = outer_layer_visaable
        + vis_matrix.elements_row_major_iter().fold(0, |mut acc, c| {
            if *c {
                acc += 1;
            }
            acc
        });

    println!("{score}")
}
