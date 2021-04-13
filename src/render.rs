use crate::board::Board;
use crate::board::BOARD_SIZE;

use std::error::Error;
use std::io::{stdout, Stdout, Write};
use termion::{clear, color};
use termion::screen::AlternateScreen;
use termion::color::{Color, LightWhite, Green, Reset, Fg, Red, LightYellow, Rgb, White, Blue, LightGreen};
use termion::event::Key::F;

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

        let cell_color = Rgb(127,127,127);
        let box_color = LightGreen;
        let clear_color = Reset;

        writeln!(self.screen, "    1   2   3   4   5   6   7   8   9  ")?;
        for row in 0..BOARD_SIZE {

            let color:&dyn Color = if row % 3 == 0 { &box_color } else { &cell_color };
            writeln!(self.screen, "{}  +---+---+---+---+---+---+---+---+---+{}", Fg(color), Fg(Reset))?;

            write!(self.screen, "{} ", row + 1)?;
            for col in 0..BOARD_SIZE {
                let c = board.get_value(row, col);
                let c = if c == 0u8 {
                    "".to_string()
                } else {
                    c.to_string()
                };
                let color:&dyn Color = if col % 3 == 0 { &box_color } else { &cell_color };
                write!(self.screen, "{}|{}", Fg(color), Fg(Reset))?;
                write!(self.screen, "{:^width$}", c, width = 3)?;
            }
            writeln!(self.screen, "{}|{}", Fg(box_color), Fg(Reset))?;
        }
        write!(self.screen, "{}  +---+---+---+---+---+---+---+---+---+{}", Fg(box_color), Fg(Reset)).unwrap();
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
