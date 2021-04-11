use board::Board;
use input::Command;

use crate::generator::BasicGenerator;
use crate::render::{ConsoleRender, Render};

mod board;
mod generator;
mod input;
mod render;

pub fn play() {

    let mut board = Board::new(&BasicGenerator::new());
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
            board.set_value(*row - 1, *col - 1, *val);
            (format!("Last play: [{},{}] = {}", row, col, val), false)
        }
        Command::Quit => (String::from("Good Bye !!!"), true),
    }
}
