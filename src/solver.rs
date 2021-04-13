use crate::board::{Board, BOARD_SIZE};

use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

pub trait Solver {
    fn solve(&self, board: &mut Board) -> bool;
}

pub struct SimpleSolver;

impl Solver for SimpleSolver {
    fn solve(&self, board: &mut Board) -> bool {
        if Self::fill_cell(&mut board.clone(), 0, 0) {
            return Self::fill_cell(board, 0, 0);
        }
        return false;
    }
}

impl SimpleSolver {

    pub fn new() -> SimpleSolver {
        SimpleSolver
    }

    fn fill_cell(board: &mut Board, row: usize, col: usize) -> bool {

        if row == BOARD_SIZE || col == BOARD_SIZE {
            return true;
        }

        if board.get_value(row, col) != 0 {
            let next_cell = Self::next_cell(row, col);
            return Self::fill_cell(board, next_cell.0, next_cell.1);
        }

        let mut available_values = board.get_available_values(row, col);
        available_values.shuffle(&mut thread_rng());

        for val in available_values {
            board.set_value(row, col, val).unwrap();

            let next_cell = Self::next_cell(row, col);
            if Self::fill_cell(board, next_cell.0, next_cell.1) {
                return true;
            }

            board.clear_value(row, col);
        }

        return false;
    }

    fn next_cell(row: usize, col: usize) -> (usize, usize) {
        let cell = row * BOARD_SIZE + col + 1;
        (cell / BOARD_SIZE, cell % BOARD_SIZE)
    }

}
