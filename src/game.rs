use crate::board::Board;
use crate::generator::{BasicGenerator, Difficulty, Generator};
use crate::input;
use crate::render::{ConsoleRender, Render};
use crate::solver::{SimpleSolver, Solver};
use std::time::Instant;

pub struct Game {
    title: String,
    board: Board,
    message: String,
    start_time: Instant,
    quit: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            title: String::from("Sudoku"),
            board: BasicGenerator::new(Difficulty::Easy).generate(),
            message: String::from("Welcome"),
            start_time: Instant::now(),
            quit: false,
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_message(&mut self, msg: String) {
        self.message = msg
    }

    pub fn start_time(&self) -> &Instant {
        &self.start_time
    }

    pub fn new_grid(&mut self, difficulty: Difficulty) {
        self.board = BasicGenerator::new(difficulty).generate();
        self.start_time = Instant::now();
    }

    pub fn fill_cell(&mut self, row: usize, col: usize, val: u8) -> Result<(), String> {
        self.board.set_value(row, col, val)
    }

    pub fn solve(&mut self) -> bool {
        let solved = SimpleSolver::new().solve(&mut self.board);
        if solved {
            self.board.freeze();
        }
        solved
    }

    pub fn end(&mut self) {
        self.board.freeze()
    }

    pub fn reset(&mut self) {
        self.board.reset();
        self.start_time = Instant::now();
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn play(&mut self) {
        let mut console = ConsoleRender::new();

        console.render(self);
        console.render(self); // workaround to clean the screen

        while !self.quit {
            let command = input::read_input_command();
            command(self);

            console.render(self);
        }
    }
}
