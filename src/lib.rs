use board::Board;
use input::Command;

use crate::generator::BasicGenerator;
use crate::render::{ConsoleRender, Render};
use crate::solver::{SimpleSolver, Solver};

mod board;
mod generator;
mod input;
mod render;
mod solver;

pub fn play() {
    //let mut board = Board::new(Some(&BasicGenerator::new()));
    let mut board = Board::new(None);
    let mut console = ConsoleRender::new();

    console.render(&board, &String::from("Welcome"));
    console.render(&board, &String::from("Welcome")); // workaround to clean the screen

    loop {
        let command = input::read_input();
        let (msg, exit) = update(&mut board, &command);
        if exit {
            return;
        }

        console.render(&board, &msg);
    }
}

fn update(board: &mut Board, command: &Command) -> (String, bool) {
    match command {
        Command::WrongInput(err) => (format!("Wrong input: {}", err), false),
        Command::WriteDigit(row, col, val) => {
            if let Err(e) = board.set_value(*row - 1, *col - 1, *val) {
                (
                    format!("Value {} not set in [{},{}]: {}", row, col, val, e),
                    false,
                )
            } else {
                (format!("Last play: [{},{}] = {}", row, col, val), false)
            }
        }
        Command::Solve => {
            let solved = SimpleSolver::new().solve(board);
            if solved {
                (String::from("Solved !!!"), false)
            } else {
                (String::from("No solution found !!!"), false)
            }
        },
        Command::Quit => (String::from("Good Bye !!!"), true),
    }
}
