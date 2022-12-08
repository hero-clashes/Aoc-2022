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


    let mut vis_matrix = Array2D::filled_with(0, matrix.num_rows(), matrix.num_rows());
    for x in 0..vis_matrix.num_columns() {
        for y in 0..vis_matrix.num_rows() {
            let hight = matrix[(y, x)];
            let right_check = matrix.row_iter(y).skip(x + 1).fold((true,0), |mut acc, cur_hight| {
                if !acc.0 {
                    return acc;
                }
                if *cur_hight >= hight {
                    acc.0 = false;
                };
                acc.1+=1;
                acc
            });
            let mut left_check = matrix.row_iter(y).collect::<Vec<_>>();
            left_check.resize(x, &0);
            left_check.reverse();
            let left_check = left_check.iter().fold((true,0), |mut acc, cur_hight| {
                if !acc.0 {
                    return acc;
                }
                if **cur_hight >= hight {
                    acc.0 = false;
                };
                acc.1+= 1;
                acc
            });
            let down_check = matrix
                .column_iter(x)
                .skip(y + 1)
                .fold((true,0), |mut acc, cur_hight| {
                    if !acc.0 {
                        return acc;
                    }
                    if *cur_hight >= hight {
                        acc.0 = false;
                    };
                    acc.1+= 1;
                    acc
                });
            let mut up_check = matrix.column_iter(x).collect::<Vec<_>>();
            up_check.resize(y, &0);
            up_check.reverse();
            let up_check = up_check.iter().fold((true,0), |mut acc, cur_hight| {
                if !acc.0 {
                    return acc;
                }
                if **cur_hight >= hight {
                    acc.0 = false;
                };
                acc.1+= 1;
                acc
            });
            vis_matrix[(y, x)] = right_check.1 * left_check.1 * up_check.1 * down_check.1;
        }
    }


    let score = vis_matrix.elements_column_major_iter().max().unwrap();

    println!("{score}")
}
