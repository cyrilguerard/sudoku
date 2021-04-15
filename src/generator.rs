use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::board::{Board, BOARD_SIZE};
use crate::solver::{SimpleSolver, Solver};

const EASY: u8 = 38;
const MEDIUM: u8 = 30;
const HARD: u8 = 25;
const EXPERT: u8 = 23;

#[derive(Copy, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}

impl Into<&'static str> for Difficulty {
    fn into(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Medium => "Medium",
            Difficulty::Hard => "Hard",
            Difficulty::Expert => "Expert",
        }
    }
}

pub trait Generator {
    fn generate(&self) -> Board;
}

pub struct BasicGenerator {
    nb_filled_cell: u8,
}

impl Generator for BasicGenerator {
    fn generate(&self) -> Board {
        let mut board = Board::new();
        SimpleSolver::new().solve(&mut board); // always solvable

        let total_cells = BOARD_SIZE * BOARD_SIZE;
        let mut cells: Vec<usize> = (0..total_cells).collect();
        cells.shuffle(&mut thread_rng());

        cells
            .iter()
            .take(total_cells - self.nb_filled_cell as usize)
            .for_each(|pos| {
                board
                    .clear_value(pos / BOARD_SIZE, pos % BOARD_SIZE)
                    .unwrap()
            });

        board.freeze();
        board
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
