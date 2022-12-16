use array2d::Array2D;
use core::num;
use inpt::{inpt, Inpt};
use itertools::Itertools;
use pathfinding::directed::astar;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::Peekable;
use std::ops::ControlFlow;
use std::str::Lines;

#[derive(Clone, Debug)]
enum tile {
    elevation(i32),
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
struct grid_loc(i32, i32);

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let x_width = contents.lines().nth(0).unwrap().len();
    let y_hight = contents.lines().count();
    let mut matrix = vec![vec![tile::elevation(0); y_hight]; x_width];
    let mut start = grid_loc::default();
    let mut goal = grid_loc::default();
    for (row, line) in contents.lines().enumerate() {
        for (column, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = grid_loc(column as i32, row as i32);

                matrix[column][row] = tile::elevation(('a' as i32 - 'a' as i32) + 1);
            } else if ch == 'E' {
                goal = grid_loc(column as i32, row as i32);

                matrix[column][row] = tile::elevation(('z' as i32 - 'a' as i32) + 1);
            } else {
                let elev = (ch as i32 - 'a' as i32) + 1;
                matrix[column][row] = tile::elevation(elev);
            }
        }
    }
    let mut lowest_steps = 99999999;
    for y in 0..y_hight {
        for x in 0..x_width {
            match matrix[x][y]{
                tile::elevation(h) => {
                    if h == 1 {
                        let path = astar::astar(
                            &grid_loc(x as i32,y as i32),
                            |current| {
                                let mut output = Vec::new();
                                let hight = match matrix[current.0 as usize][current.1 as usize] {
                                    tile::elevation(s) => s,
                                };
                                for dx in [-1, 1] {
                                    let x = current.0 + dx;
                                    let y = current.1;
                                    if let ControlFlow::Break(_) =
                                        fun_name(x, x_width, y, y_hight, &matrix, &mut output, hight)
                                    {
                                        continue;
                                    }
                                }
                                for dy in [-1, 1] {
                                    let x = current.0;
                                    let y = current.1 + dy;
                                    if let ControlFlow::Break(_) =
                                        fun_name(x, x_width, y, y_hight, &matrix, &mut output, hight)
                                    {
                                        continue;
                                    }
                                }
                                return output;
                            },
                            |a| return (a.0 - goal.0).abs() + (a.1 - goal.1).abs(),
                            |a| *a == goal,
                        );
                        if path.is_some() {
                            let num = path.unwrap().1;
                            if num < lowest_steps{
                                lowest_steps = num;
                            }
                        }
                    }
                },
            }
        }
    }

    println!("{}", lowest_steps);
}

fn fun_name(
    x: i32,
    x_width: usize,
    y: i32,
    y_hight: usize,
    matrix: &Vec<Vec<tile>>,
    output: &mut Vec<(grid_loc, i32)>,
    hight: i32,
) -> ControlFlow<()> {
    if x as usize >= x_width || y as usize >= y_hight || x < 0 || y < 0 {
        return ControlFlow::Break(());
    }
    match matrix[x as usize][y as usize] {
        tile::elevation(current_h) => {
            let dif = current_h - hight;
            if dif <= 1 {
                output.push((grid_loc(x, y), 1))
            }
        }
    }
    ControlFlow::Continue(())
}
