use std::io;

use crate::board::BOARD_SIZE;
use crate::input::Command::{Quit, WriteDigit, WrongInput};

pub enum Command {
    WrongInput(String),
    WriteDigit(usize, usize, u8),
    Quit,
}

pub fn read_input() -> Command {
    let mut line = String::new();
    if let Err(_) = io::stdin().read_line(&mut line) {
        return WrongInput(String::from("Unable to read input"));
    }

    let inputs: Vec<_> = line.trim().split(" ").into_iter().collect();

    match inputs.len() {
        1 => parse_quit(inputs[0]),
        3 => parse_write_digit_input(&inputs),
        _ => WrongInput(String::from("Unknown command")),
    }
}

fn parse_write_digit_input(inputs: &Vec<&str>) -> Command {
    assert_eq!(inputs.len(), 3);

    if let Ok(row) = read_one_digit(inputs[0], 1) {
        if let Ok(col) = read_one_digit(inputs[1], 1) {
            if let Ok(val) = read_one_digit(inputs[2], 0) {
                return WriteDigit(row as usize, col as usize, val);
            }
        }
    }

    WrongInput(format!(
        "Expected values: [1-{}] [1-{}] [0-{}]",
        BOARD_SIZE, BOARD_SIZE, BOARD_SIZE
    ))
}

fn parse_quit(input: &str) -> Command {
    if String::from(input).to_lowercase().eq("q") {
        Quit
    } else {
        WrongInput(String::from("Unknown command"))
    }
}

fn read_one_digit(input: &str, min: u8) -> Result<u8, ()> {
    if let Ok(val) = input.parse::<u8>() {
        if (min..=BOARD_SIZE as u8).contains(&val) {
            return Ok(val);
        }
    }
    Result::Err(())
}
