use std::collections::HashMap;
use std::io;

use crate::board::BOARD_SIZE;
use crate::game::Game;
use crate::generator::Difficulty;

pub type InputCommand = Box<dyn FnOnce(&mut Game) -> ()>;
pub type ParseCommand = fn(Vec<&str>) -> InputCommand;

lazy_static! {
    static ref COMMANDS: HashMap<&'static str, ParseCommand> = {
        let mut m = HashMap::<&'static str, ParseCommand>::new();
        m.insert("new", cmd_new);
        m.insert("clear", cmd_clear_cell_value);
        m.insert("solve", cmd_solve);
        m.insert("reset", cmd_reset);
        m.insert("help", cmd_show_help);
        m.insert("quit", cmd_quit);
        m
    };
}

pub fn read_input_command() -> InputCommand {
    let mut line = String::new();
    if let Err(_) = io::stdin().read_line(&mut line) {
        return cmd_error(vec!["Unknown command"]);
    }

    let inputs: Vec<_> = line.trim().split(" ").into_iter().collect();
    if let Some(parse_command) = COMMANDS.get(inputs[0]) {
        parse_command(inputs)
    } else if inputs.len() == 3 {
        cmd_write_cell_value(inputs)
    } else {
        cmd_error(vec!["Unknown command"])
    }
}

fn cmd_error(args: Vec<&'static str>) -> InputCommand {
    assert_eq!(args.len(), 1);
    let message = args[0];
    Box::new(move |game| {
        game.set_message(format!("Error: {}", message));
    })
}

fn cmd_new(args: Vec<&str>) -> InputCommand {
    let difficulty = args
        .get(1)
        .map(|s| s.to_lowercase())
        .map(|s| match s.as_str() {
            "easy" => Some(Difficulty::Easy),
            "medium" => Some(Difficulty::Medium),
            "hard" => Some(Difficulty::Hard),
            "expert" => Some(Difficulty::Expert),
            _ => None,
        })
        .flatten();

    if let Some(d) = difficulty {
        Box::new(move |game| {
            game.new_grid(d);
            game.set_message(String::new());
        })
    } else {
        cmd_error(vec!["Usage: new [easy|medium|hard|expert]"])
    }
}

fn cmd_write_cell_value(args: Vec<&str>) -> InputCommand {
    if args.len() == 3 {
        if let Ok(row) = read_one_digit(args[0]) {
            if let Ok(col) = read_one_digit(args[1]) {
                if let Ok(val) = read_one_digit(args[2]) {
                    return Box::new(move |game| {
                        match game.fill_cell((row - 1) as usize, (col - 1) as usize, val) {
                            Ok(_) => {
                                if game.board().is_solved() {
                                    let seconds = game.start_time().elapsed().as_secs();
                                    game.set_message(format!(
                                        "Congratulations !!! You solved this Sudoku in {}m{}s.",
                                        seconds / 60,
                                        seconds % 60
                                    ));
                                    game.end();
                                } else {
                                    game.set_message(format!("[{},{}] = {} done", row, col, val))
                                }
                            }
                            Err(e) => game.set_message(format!(
                                "Error: {} (input: [{},{}] = {})",
                                e, row, col, val
                            )),
                        };
                    });
                }
            }
        }
    }
    cmd_error(vec!["Usage: <row:[1-9]> <col:[1-9]> <val:[1-9]>"])
}

fn cmd_clear_cell_value(args: Vec<&str>) -> InputCommand {
    if args.len() == 3 {
        if let Ok(row) = read_one_digit(args[1]) {
            if let Ok(col) = read_one_digit(args[2]) {
                return Box::new(move |game| {
                    match game.fill_cell((row - 1) as usize, (col - 1) as usize, 0) {
                        Ok(_) => game.set_message(format!("[{},{}] cleared", row, col)),
                        Err(e) => {
                            game.set_message(format!("Error: {} (clear: [{},{}])", e, row, col))
                        }
                    };
                });
            }
        }
    }
    cmd_error(vec!["Usage: clear <row:[1-9]> <col:[1-9]>"])
}

fn cmd_solve(_args: Vec<&str>) -> InputCommand {
    Box::new(|game| {
        let solved = game.solve();
        if solved {
            game.set_message(String::from("Solved."));
        } else {
            game.set_message(String::from("No solution found."));
        }
    })
}

fn cmd_reset(_args: Vec<&str>) -> InputCommand {
    Box::new(|game| {
        game.reset();
        game.set_message(String::new());
    })
}

fn cmd_show_help(_args: Vec<&str>) -> InputCommand {
    Box::new(|game| {
        game.set_message(String::from(
            "[Help] Available commands: \
        <row> <col> <val> | \
        clear <row> <col> | \
        new <difficulty> | \
        solve | \
        reset | \
        help | \
        quit",
        ));
    })
}

fn cmd_quit(_args: Vec<&str>) -> InputCommand {
    Box::new(|game| {
        game.quit();
    })
}

fn read_one_digit(input: &str) -> Result<u8, ()> {
    if let Ok(val) = input.parse::<u8>() {
        if (1..=BOARD_SIZE as u8).contains(&val) {
            return Ok(val);
        }
    }
    Result::Err(())
}
