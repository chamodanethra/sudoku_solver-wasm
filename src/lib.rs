extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::convert::TryInto;

struct Grid {
    grid: [[i32; 9]; 9],
    completed: bool,
    filled_order: Vec<(usize, usize)>,
    remaining_digits_set: Vec<Vec<usize>>,
}

impl Grid {
    fn get_best_square(&self) -> ((usize, usize), Vec<usize>) {
        let mut max_square = (9, 9);
        let mut max_square_digits: Vec<i32> = Vec::new();
        'outermost:for i in 0..9 {
            for j in 0..9 {
                if self.grid[i][j] == 0 {
                    let mut current_square_digits: Vec<i32> = Vec::new();
                    for k in 0..9 {
                        if self.grid[k][j] != 0 {
                            current_square_digits.push(self.grid[k][j]);
                        }
                    }

                    'outer1: for k in 0..9 {
                        if self.grid[i][k] != 0 {
                            for num in &current_square_digits {
                                if self.grid[i][k] == *num {
                                    continue 'outer1;
                                }
                            }
                            current_square_digits.push(self.grid[i][k]);
                        }
                    }

                    if i != 9 {
                        for m in ((i/3)*3 as usize)..((i/3)*3 + 3 as usize) {
                            'middle: for n in ((j/3)*3 as usize)..((j/3)*3 + 3 as usize) {
                                if self.grid[m][n] != 0 {
                                    for num in &current_square_digits {
                                        if self.grid[m][n] == *num {
                                            continue 'middle;
                                        }
                                    }
                                    current_square_digits.push(self.grid[m][n]);
                                }
                            }
                        }
                    }

                    if current_square_digits.len() > max_square_digits.len() {
                        max_square_digits = current_square_digits;
                        max_square = (i, j);
                        if max_square_digits.len() == 8 {
                            break 'outermost;
                        }
                    }
                }
            }
        }

        let mut available_digits: Vec<usize> = Vec::new();
        'outer3: for l in 1..10 {
            for k in &max_square_digits {
                if *k == l {
                    continue 'outer3;
                }
            }
            available_digits.push(l as usize);
        }
        (max_square, available_digits)
    }

    fn write_data(&mut self, i: usize, j: usize, num: i32, remaining_digits: Vec<usize>) {
        self.grid[i][j] = num;
        self.filled_order.push((i, j));
        self.remaining_digits_set.push(remaining_digits);
    }

    fn erase_data(&mut self) {
        loop {
            let (i, j) = self.filled_order[self.filled_order.len() - 1];
            self.remaining_digits_set.pop();
            self.filled_order.pop();
            self.grid[i][j] = 0;
            if self.remaining_digits_set[self.remaining_digits_set.len() - 1].len() > 0 {
                let (i, j) = self.filled_order[self.filled_order.len() - 1];
                let modifiable_remaining_digits =
                    self.remaining_digits_set[self.remaining_digits_set.len() - 1].clone();
                self.remaining_digits_set.pop();
                self.grid[i][j] = modifiable_remaining_digits[0].try_into().unwrap();
                self.remaining_digits_set
                    .push(modifiable_remaining_digits[1..].to_vec());
                break;
            }
        }
    }
}

#[wasm_bindgen]
pub fn calculate(data: &str) ->JsValue {
    let split = data.lines();
    let vec = split.collect::<Vec<&str>>();
    let mut i = 0;
    let mut grid = [[0; 9]; 9];

    for row in &vec {
        for j in 0..9 {
            grid[i][j] = row[j..(j + 1)]
                .parse()
                .expect("Please type a number!");
        }
        i = i + 1;
    }

    let output = sudoku_solver(grid);
    return JsValue::from_serde(&output).unwrap();
}

fn sudoku_solver(grid: [[i32; 9]; 9]) -> String {
    let mut sudoku_grid = Grid {
        grid,
        completed: false,
        filled_order: Vec::new(),
        remaining_digits_set: Vec::new(),
    };

    while !(&mut sudoku_grid).completed {
        let ((i, j), available_digits) = (&mut sudoku_grid).get_best_square();

        if i != 9 {
            if available_digits.len() == 0 {
                (&mut sudoku_grid).erase_data();
                continue;
            } else {
                let mut remaining_digits = Vec::new();
                let remove_digit = available_digits[0];
                for r in 1..available_digits.len() {
                    remaining_digits.push(available_digits[r]);
                }

                (&mut sudoku_grid).write_data(i, j, remove_digit as i32, remaining_digits);
            }
        } else {
            (&mut sudoku_grid).completed = true;
        }
    }

    let mut output = String::with_capacity(81);
    for digit in 0..81 {
        output.push_str(&sudoku_grid.grid[digit / 9][digit % 9].to_string());
        if digit % 9 == 8 {
            output.push_str("\n");
        }
    }

    return output;
}