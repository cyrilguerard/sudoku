use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::board::{Board, BOARD_SIZE};

pub trait Generator {
    fn fill(&self, board: &mut Board);
}

pub struct BasicGenerator;

impl Generator for BasicGenerator {
    fn fill(&self, board: &mut Board) {
        Self::fill_cell(board, 0, 0);
    }
}

impl BasicGenerator {
    pub fn new() -> BasicGenerator {
        BasicGenerator
    }

    fn fill_cell(board: &mut Board, row: usize, col: usize) -> bool {
        if row == BOARD_SIZE || col == BOARD_SIZE {
            return true;
        }

        let mut available_values = board.get_available_values(row, col);
        if !available_values.is_empty() {
            available_values.shuffle(&mut thread_rng());

            for val in available_values {
                board.set_value(row, col, val).unwrap();

                let cell = row * BOARD_SIZE + col + 1;
                let valid = Self::fill_cell(board, cell / BOARD_SIZE, cell % BOARD_SIZE);
                if valid {
                    return true;
                }

                board.clear_value(row, col);
            }
        }

        return false;
    }
}
