use crate::board::Board;
use crate::generator::{BasicGenerator, Difficulty, Generator};
use crate::input;
use crate::render::{ConsoleRender, Render};
use crate::solver::{SimpleSolver, Solver};
use std::time::Instant;

const HELP: &'static [&'static str] = &[
    "   <R> <C> <V>: Set the value V in the cell at row R and column C.",
    " clear <R> <C>: Clear the value in the cell at row R and column C.",
    "       new <D>: Start a new sudoku with difficulty D in [easy, medium, hard, expert].",
    "         reset: Reset the current sudoku.",
    "         solve: Solve the current sudoku.",
    "          quit: Quit the game."
];

pub struct Game {
    board: Board,
    message: String,
    start_time: Instant,
    quit: bool,
    headers: Vec<String>,
    footers: Vec<String>,
}

impl Game {

    pub fn new() -> Game {
        let mut game = Game {
            board: Board::new(),
            message:  String::from("Welcome"),
            start_time: Instant::now(),
            quit: false,
            headers: vec![],
            footers: HELP.iter().map(|str| String::from(*str)).collect()
        };
        game.new_grid(Difficulty::Easy);
        game
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    pub fn start_time(&self) -> &Instant {
        &self.start_time
    }

    pub fn headers(&self) -> &Vec<String> {
        &self.headers
    }

    pub fn footers(&self) -> &Vec<String> {
        &self.footers
    }

    pub fn new_grid(&mut self, difficulty: Difficulty) {
        self.headers= vec![
            String::from("Sudoku"),
            String::new(),
            format!("Difficulty: {}", Into::<&str>::into(difficulty))];
        self.start_time = Instant::now();
        self.board = BasicGenerator::new(difficulty).generate();
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
