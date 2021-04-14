use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::board::{Board, BOARD_SIZE};
use crate::solver::{SimpleSolver, Solver};

const EASY: u8 = 38;
const MEDIUM: u8 = 30;
const HARD: u8 = 25;
const EXPERT: u8 = 23;

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

pub trait Generator {
    fn fill(&self, board: &mut Board);
}

pub struct BasicGenerator {
    nb_filled_cell: u8,
}

impl Generator for BasicGenerator {
    fn fill(&self, board: &mut Board) {
        SimpleSolver::new().solve(board); // TODO: check if solvalble

        let total_cells = BOARD_SIZE * BOARD_SIZE;
        let mut cells: Vec<usize> = (0..total_cells).collect();
        cells.shuffle(&mut thread_rng());

        cells
            .iter()
            .take(total_cells - self.nb_filled_cell as usize)
            .for_each(|pos| board.clear_value(pos / BOARD_SIZE, pos % BOARD_SIZE));
    }
}

impl BasicGenerator {
    pub fn new(difficulty: Difficulty) -> BasicGenerator {
        let nb_filled_cell = match difficulty {
            Difficulty::Easy => EASY,
            Difficulty::Medium => MEDIUM,
            Difficulty::Hard => HARD,
            Difficulty::Expert => EXPERT,
        };
        BasicGenerator { nb_filled_cell }
    }
}
