use board::Board;
use input::Command;

use crate::generator::{BasicGenerator, Difficulty};
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
        let (new_board, msg, exit) = update(&mut board, &command);
        if exit {
            return;
        }

        if new_board.is_some() {
            board = new_board.unwrap();
        }

        console.render(&board, &msg);
    }
}

fn update(board: & mut Board, command: &Command) -> (Option<Board>, String, bool) {
    match command {
        Command::WrongInput(err) => (None, format!("Wrong input: {}", err), false),
        Command::WriteDigit(row, col, val) => {
            if let Err(e) = board.set_value(*row - 1, *col - 1, *val) {
                (
                    None,
                    format!("Value {} not set in [{},{}]: {}", row, col, val, e),
                    false,
                )
            } else {
                (None, format!("Last play: [{},{}] = {}", row, col, val), false)
            }
        }
        Command::Solve => {
            let solved = SimpleSolver::new().solve(board);
            if solved {
                (None, String::from("Solved !!!"), false)
            } else {
                (None, String::from("No solution found !!!"), false)
            }
        },
        Command::NewEasy => {
            let new_board = Board::new(Some(&BasicGenerator::new(Difficulty::Easy)));
            (Some(new_board), String::from("New game: Easy !!!"), false)
        },
        Command::NewMedium => {
            let new_board = Board::new(Some(&BasicGenerator::new(Difficulty::Medium)));
            (Some(new_board), String::from("New game: Medium !!!"), false)
        },
        Command::NewHard => {
            let new_board = Board::new(Some(&BasicGenerator::new(Difficulty::Hard)));
            (Some(new_board), String::from("New game: Hard !!!"), false)
        }
        ,
        Command::NewExpert => {
            let new_board = Board::new(Some(&BasicGenerator::new(Difficulty::Expert)));
            (Some(new_board), String::from("New game: Expert !!!"), false)
        }
        Command::Quit => (None, String::from("Good Bye !!!"), true),
    }
}
