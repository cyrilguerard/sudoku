use std::error::Error;
use std::io::{stdout, Stdout, Write};

use termion::clear;
use termion::color::*;
use termion::screen::AlternateScreen;

use crate::board::{Board, BOARD_BOX_SIZE};
use crate::board::BOARD_SIZE;
use crate::game::Game;
use core::cmp;

const SCREEN_WIDTH: usize = 80;

pub trait Render {
    fn render(&mut self, game: &Game);
}

pub struct ConsoleRender {
    screen: AlternateScreen<Stdout>,
}

#[derive(Copy, Clone)]
enum Align {
    LEFT,
    CENTER,
    //RIGHT
}

impl Render for ConsoleRender {
    fn render(&mut self, game: &Game) {
        self.write(game).unwrap()
    }
}

impl ConsoleRender {
    pub fn new() -> ConsoleRender {
        ConsoleRender {
            screen: AlternateScreen::from(stdout()),
        }
    }

    fn write(&mut self, game: &Game) -> Result<(), Box<dyn Error>> {
        writeln!(self.screen, "{}", clear::All)?;
        self.screen.flush()?;
        self.write_line_break()?;
        self.write_line_break()?;
        self.write_lines(game.headers(), Align::CENTER)?;
        self.write_line_break()?;
        self.write_line_break()?;
        self.write_board(game.board())?;
        self.write_line_break()?;
        self.write_line_break()?;
        self.write_line(&game.message(), Align::CENTER, true)?;
        self.write_line_break()?;
        self.write_line_break()?;
        self.write_lines(&game.footers(), Align::LEFT)?;
        self.write_line_break()?;
        self.write_line_break()?;
        self.write_line("sudoku$ ", Align::LEFT, false)?;
        self.screen.flush()?;
        Ok(())
    }

    fn write_board_header(&mut self, color: &dyn Color, margin_width: usize) -> Result<(), Box<dyn Error>> {
        write!(self.screen, "{}", Fg(color))?;
        write!(self.screen, "{:width$}", "", width = margin_width)?;
        write!(self.screen, " {:^3}", " ")?;
        for val in 1..=BOARD_SIZE {
            write!(self.screen, " {:^3}", val)?;
        }
        write!(self.screen, " {:^3} ", " ")?;
        write!(self.screen, "{:width$}", "", width = margin_width)?;
        write!(self.screen, "{}", Fg(Reset))?;
        writeln!(self.screen)?;
        Ok(())
    }

    fn write_board_line(&mut self, main_color: &dyn Color, sub_color: &dyn Color, margin_width: usize) -> Result<(), Box<dyn Error>> {
        write!(self.screen, "{}", Fg(main_color))?;
        write!(self.screen, "{:width$}", "", width = margin_width)?;
        write!(self.screen, " {:^3}", " ")?;
        for col in 0..BOARD_SIZE {
            let color = if col % BOARD_BOX_SIZE == 0 { sub_color } else { main_color };
            write!(self.screen, "{}{}", Fg(color), "+", )?;
            write!(self.screen, "{}{}", Fg(main_color), "---")?;
        }
        write!(self.screen, "{}+{}{:^3} ", Fg(sub_color), Fg(main_color), " ")?;
        write!(self.screen, "{:width$}", "", width = margin_width)?;
        write!(self.screen, "{}", Fg(Reset))?;
        writeln!(self.screen)?;
        Ok(())
    }

    fn write_board_row(&mut self, board: &Board, row: usize, box_color: &dyn Color, cell_color: &dyn Color, margin_width: usize) -> Result<(), Box<dyn Error>> {
        write!(self.screen, "{:width$}", "", width = margin_width)?;
        write!(self.screen, " {}{:^3}", Fg(cell_color), row + 1)?;
        for col in 0..BOARD_SIZE {
            let color = if col % BOARD_BOX_SIZE == 0 { box_color } else { cell_color };
            let val = board.get_value(row, col).map_or(String::new(), |val| val.to_string());
            write!(self.screen, "{}{}", Fg(color), "|", )?;
            if board.is_fixed_value(row, col) {
                write!(self.screen, "{}{:^3}", Fg(Rgb(102, 178, 255)), val)?;
            } else {
                write!(self.screen, "{}{:^3}", Fg(Reset), val)?;
            }
        }
        write!(self.screen, "{}|{}{:^3} ", Fg(box_color), Fg(cell_color), row + 1)?;
        write!(self.screen, "{:width$}", "", width = margin_width)?;
        write!(self.screen, "{}", Fg(Reset))?;
        writeln!(self.screen)?;
        Ok(())
    }

    fn write_board(&mut self, board: &Board) -> Result<(), Box<dyn Error>> {

        let box_color = LightGreen;
        let cell_color = Rgb(127, 127, 127);

        let column_size = 3;
        let board_width = (BOARD_SIZE + 2) * column_size + (BOARD_SIZE + 2) + 1;
        let margin_width = cmp::max(0, SCREEN_WIDTH - board_width) / 2;

        self.write_board_header(&cell_color, margin_width)?;
        for row in 0..BOARD_SIZE {
            let color: &dyn Color = if row % 3 == 0 { &box_color} else { &cell_color };
            self.write_board_line(&color, &box_color, margin_width)?;
            self.write_board_row(board, row, &box_color, &cell_color, margin_width)?;
        }
        self.write_board_line(&box_color, &box_color, margin_width)?;
        self.write_board_header(&cell_color, margin_width)?;
        Ok(())
    }

    fn write_lines(&mut self, lines: &Vec<String>, align: Align) -> Result<(), Box<dyn Error>> {
        for line in lines {
            self.write_line(line, align, true)?;
        }
        Ok(())
    }

    fn write_line(&mut self, text: &str, align: Align, line_break: bool) -> Result<(), Box<dyn Error>> {
        if text.to_lowercase().contains("error") {
            write!(self.screen, "{}", Fg(LightRed))?;
        }
        match align {
            Align::LEFT => write!(self.screen, "{}", text)?,
            Align::CENTER => write!(self.screen, "{:^width$}", text, width = SCREEN_WIDTH)?,
            //Align::RIGHT => write!(self.screen, "{:>width$}", text, width = SCREEN_WIDTH)?
        }
        if line_break {
            self.write_line_break()?;
        }
        write!(self.screen, "{}", Fg(Reset))?;
        Ok(())
    }

    fn write_line_break(&mut self) -> Result<(), Box<dyn Error>> {
        writeln!(self.screen, "")?;
        Ok(())
    }

}
