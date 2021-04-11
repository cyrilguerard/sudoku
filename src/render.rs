use crate::board::Board;
use crate::board::BOARD_SIZE;

use std::error::Error;
use std::io::{stdout, Stdout, Write};
use termion::clear;
use termion::screen::AlternateScreen;

pub trait Render {
    fn render(&mut self, board: &Board, msg: &String);
}

pub struct ConsoleRender {
    screen: AlternateScreen<Stdout>,
}

impl Render for ConsoleRender {
    fn render(&mut self, board: &Board, msg: &String) {
        self.write(board, msg).unwrap();
    }
}

impl ConsoleRender {
    pub fn new() -> ConsoleRender {
        ConsoleRender {
            screen: AlternateScreen::from(stdout()),
        }
    }

    fn write(&mut self, board: &Board, msg: &String) -> Result<(), Box<dyn Error>> {
        writeln!(self.screen, "{}", clear::All)?;
        self.screen.flush()?;
        self.write_header()?;
        self.write_board(board)?;
        self.write_footer(msg)?;
        self.screen.flush()?;
        Ok(())
    }

    fn write_header(&mut self) -> Result<(), Box<dyn Error>> {
        writeln!(self.screen, "")?;
        writeln!(self.screen, "                 Sudoku")?;
        writeln!(self.screen, "")?;
        writeln!(self.screen, "")?;
        Ok(())
    }

    fn write_board(&mut self, board: &Board) -> Result<(), Box<dyn Error>> {
        writeln!(self.screen, "    1   2   3   4   5   6   7   8   9  ")?;
        for row in 0..BOARD_SIZE {
            writeln!(self.screen, "  +---+---+---+---+---+---+---+---+---+")?;
            write!(self.screen, "{} ", row + 1)?;
            for col in 0..BOARD_SIZE {
                let c = board.get_value(row, col);
                let c = if c == 0u8 {
                    "".to_string()
                } else {
                    c.to_string()
                };
                write!(self.screen, "|{:^width$}", c, width = 3)?;
            }
            writeln!(self.screen, "|")?;
        }
        write!(self.screen, "  +---+---+---+---+---+---+---+---+---+").unwrap();
        Ok(())
    }

    fn write_footer(&mut self, msg: &String) -> Result<(), Box<dyn Error>> {
        writeln!(self.screen, "")?;
        writeln!(self.screen, "")?;
        writeln!(self.screen, "  {}", msg)?;
        writeln!(self.screen, "")?;
        writeln!(self.screen, "")?;
        write!(self.screen, "Type your next command: ")?;
        Ok(())
    }
}
