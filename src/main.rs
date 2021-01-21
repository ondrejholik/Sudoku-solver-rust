use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Sudoku {
    board: [[u8; 9]; 9],
}

impl Sudoku {
    pub fn new(sudoku: &str) -> Sudoku {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut b: [[u8; 9]; 9] = [[0; 9]; 9];

        for x in sudoku.chars() {
            // converting ex. 0 char to 0 unsigned 8bit int
            b[i][j] = (x as u8) - ('0' as u8);
            j += 1;
            if j == 9 {
                i += 1;
                j = 0;
            }
        }
        Sudoku { board: b, empty: e }
    }

    pub fn find_empty_cell(&mut self) -> (bool, usize, usize) {
        for (i, x) in self.board.iter().enumerate() {
            for (j, y) in x.iter().enumerate() {
                if *y == 0 {
                    return (true, i, j);
                }
            }
        }
        return (false, 0, 0);
    }

    pub fn is_valid(&mut self, row: usize, col: usize, val: u8) -> bool {
        let from_row: usize = row / 3 * 3;
        let from_col: usize = col / 3 * 3;

        for i in 0..9 {
            if self.board[row][i] == val ||
               self.board[i][col] == val ||
               // Check specific sudoku(3x3) blocks
               self.board[from_row + (i / 3)][from_col + (i % 3)] == val
            {
                return false;
            }
        }

        return true;
    }

    pub fn backtracking(&mut self) -> bool {
        let (find_empty, row, col) = self.find_empty_cell();
        if !find_empty {
            self.string();
            return true;
        }

        for i in 1..10 {
            if self.is_valid(row, col, i) {
                self.board[row][col] = i;
                if self.backtracking() {
                    return true;
                }

                self.board[row][col] = 0;
            }
        }
        return false;
    }

    pub fn string(&mut self) {
        for n in self.board.iter() {
            for x in n {
                print!("{}", x);
            }
        }
        println!();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // IO -> line(=one sudoku) by line ex. ..000300205.. (len 81)
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(ip) = line {
                let mut sud = Sudoku::new(&ip);
                sud.backtracking();
            }
        }
    }
}
