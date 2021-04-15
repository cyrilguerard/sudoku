use std::fmt::{Debug, Formatter, Result};

use crate::board::Cell::{Fixed, Free};
use bitmask::bitmask;

pub const BOARD_BOX_SIZE: usize = 3;
pub const BOARD_SIZE: usize = BOARD_BOX_SIZE * BOARD_BOX_SIZE;

bitmask! {

    mask FreeNumberMask: u16 where flags FreeNumberFlags {
        _0 = 0, // unused to easy index
        _1 = 1 << 0,
        _2 = 1 << 1,
        _3 = 1 << 2,
        _4 = 1 << 3,
        _5 = 1 << 4,
        _6 = 1 << 5,
        _7 = 1 << 6,
        _8 = 1 << 7,
        _9 = 1 << 8
    }

}

impl Default for FreeNumberMask {
    fn default() -> Self {
        FreeNumberMask::all()
    }
}

impl Debug for FreeNumberMask {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let v: Vec<u8> = (0u16..=9)
            .into_iter()
            .map(|d| self.contains(Into::<FreeNumberFlags>::into(d)))
            .map(|b| if b { 1 } else { 0 })
            .collect();
        write!(f, "{:?}", v)?;
        Result::Ok(())
    }
}

impl FreeNumberFlags {
    const VALUES: [Self; BOARD_SIZE + 1] = [
        FreeNumberFlags::_0,
        FreeNumberFlags::_1,
        FreeNumberFlags::_2,
        FreeNumberFlags::_3,
        FreeNumberFlags::_4,
        FreeNumberFlags::_5,
        FreeNumberFlags::_6,
        FreeNumberFlags::_7,
        FreeNumberFlags::_8,
        FreeNumberFlags::_9,
    ];
}

impl Debug for FreeNumberFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", Into::<u16>::into(*self))?;
        Result::Ok(())
    }
}

impl From<u16> for FreeNumberFlags {
    fn from(v: u16) -> Self {
        FreeNumberFlags::VALUES[v as usize]
    }
}

impl Into<u16> for FreeNumberFlags {
    fn into(self) -> u16 {
        FreeNumberFlags::VALUES
            .iter()
            .enumerate()
            .filter(|(_, flag)| **flag == self)
            .map(|(i, _)| i as u16)
            .next()
            .unwrap()
    }
}

#[derive(Default, Clone, Copy)]
pub struct Value(u8);

impl From<u8> for Value {
    fn from(val: u8) -> Self {
        assert!(val <= 9);
        Value(val)
    }
}

impl Into<u8> for Value {
    fn into(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Fixed(Value),
    Free(Value),
}

impl Default for Cell {
    fn default() -> Self {
        Free(0u8.into())
    }
}

impl Into<u8> for Cell {
    fn into(self) -> u8 {
        match self {
            Fixed(v) => v,
            Free(v) => v,
        }
        .into()
    }
}

impl Cell {
    fn new(val: u8, fixed: bool) -> Cell {
        let val = Value::from(val);
        if fixed {
            Fixed(val)
        } else {
            Free(val)
        }
    }

    fn lock(cell: Cell) -> Cell {
        Cell::new(cell.into(), true)
    }
}

#[derive(Default, Clone)]
pub struct Board {
    free_number_rows: [FreeNumberMask; BOARD_SIZE],
    free_number_columns: [FreeNumberMask; BOARD_SIZE],
    free_number_boxes: [FreeNumberMask; BOARD_SIZE],
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Board {
        Board::default()
    }

    pub fn freeze(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                let val: u8 = (*cell).into();
                if val != 0 {
                    *cell = Cell::lock(*cell)
                }
            }
        }
    }

    pub fn reset(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                if let Free(_) = *cell {
                    *cell = Free(0.into());
                }
            }
        }
    }

    pub fn get_value(&self, row: usize, col: usize) -> Option<u8> {
        let val: u8 = self.cells[row][col].into();
        if val == 0 {
            None
        } else {
            Some(val)
        }
    }

    pub fn is_fixed_value(&self, row: usize, col: usize) -> bool {
        if let Fixed(_) = self.cells[row][col] {
            true
        } else {
            false
        }
    }

    pub fn set_value(
        &mut self,
        row: usize,
        col: usize,
        val: u8,
    ) -> std::result::Result<(), String> {
        if self.is_fixed_value(row, col) {
            return Err(String::from("Fixed value"));
        }

        if !self.can_set_value(row, col, val) {
            return Err(String::from("Forbidden value"));
        }

        self.clear_value(row, col)?;
        self.cells[row][col] = Cell::new(val, false);

        if val != 0 {
            let flag = FreeNumberFlags::from(val as u16);
            self.free_number_rows[row].unset(flag);
            self.free_number_columns[col].unset(flag);
            self.free_number_boxes[Board::compute_box_index(row, col)].unset(flag);
        }

        Ok(())
    }

    pub fn clear_value(&mut self, row: usize, col: usize) -> std::result::Result<(), String> {
        if self.is_fixed_value(row, col) {
            return Err(String::from("Fixed value"));
        }

        if let Some(val) = self.get_value(row, col) {
            let flag = FreeNumberFlags::from(val as u16);
            self.free_number_rows[row].set(flag);
            self.free_number_columns[col].set(flag);
            self.free_number_boxes[Board::compute_box_index(row, col)].set(flag);
            self.cells[row][col] = Cell::new(0, false);
        }

        Ok(())
    }

    pub fn get_available_values(&self, row: usize, col: usize) -> Vec<u8> {
        let free_values = self.free_number_rows[row]
            & self.free_number_columns[col]
            & self.free_number_boxes[Board::compute_box_index(row, col)];

        (1..=BOARD_SIZE as u8)
            .into_iter()
            .filter(|d| free_values.contains(FreeNumberFlags::from(*d as u16)))
            .collect()
    }

    pub fn is_solved(&self) -> bool {
        self.free_number_rows
            .iter()
            .all(|mask| mask.count_ones() == 0)
    }

    fn can_set_value(&self, row: usize, col: usize, val: u8) -> bool {
        let flag = FreeNumberFlags::from(val as u16);

        self.free_number_rows[row].contains(flag)
            && self.free_number_columns[col].contains(flag)
            && self.free_number_boxes[Board::compute_box_index(row, col)].contains(flag)
    }

    fn compute_box_index(row: usize, col: usize) -> usize {
        (row / BOARD_BOX_SIZE) * BOARD_BOX_SIZE + (col / BOARD_BOX_SIZE)
    }
}

#[cfg(test)]
mod tests {

    use crate::board::{Board, FreeNumberFlags};
    use crate::solver::{SimpleSolver, Solver};

    #[test]
    fn test_fnf_from_u16() {
        (0..10u16).into_iter().for_each(|i| {
            assert_eq!(
                FreeNumberFlags::VALUES[i as usize],
                FreeNumberFlags::from(i)
            );
        })
    }

    #[test]
    fn test_fnf_into_u16() {
        (0..10u16).into_iter().for_each(|i| {
            assert_eq!(i, FreeNumberFlags::VALUES[i as usize].into());
        })
    }

    #[test]
    fn test_compute_box_index() {
        (0..3usize).into_iter().for_each(|row| {
            (0..3usize).into_iter().for_each(|col| {
                assert_eq!(0, Board::compute_box_index(row, col));
            })
        })
    }

    #[test]
    fn test_is_solved() {
        let mut board = Board::new();
        assert!(!board.is_solved());

        SimpleSolver::new().solve(&mut board);
        assert!(board.is_solved())
    }
}
