use std::fmt::{Debug, Display, Formatter, Result};

use crate::generator::Generator;
use bitmask::bitmask;

pub const BOARD_BOX_SIZE: usize = 3;
pub const BOARD_SIZE: usize = BOARD_BOX_SIZE * BOARD_BOX_SIZE;

bitmask! {

    mask FreeNumberMask: u16 where flags FreeNumberFlags {
        _0 = 0,
        _1 = 1,
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
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
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

#[derive(Default)]
pub struct Board {
    free_number_rows: [FreeNumberMask; BOARD_SIZE],
    free_number_columns: [FreeNumberMask; BOARD_SIZE],
    free_number_boxes: [FreeNumberMask; BOARD_SIZE],
    cells: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {

    pub fn new(generator: &dyn Generator) -> Board {
        let mut board = Board::default();
        generator.fill(&mut board);
        board
    }

    pub fn get_value(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    pub fn set_value(&mut self, row: usize, col: usize, val: u8) {
        self.clear_value(row, col);
        self.cells[row][col] = val;

        if val != 0 {
            let flag = FreeNumberFlags::from(val as u16);
            self.free_number_rows[row].unset(flag);
            self.free_number_columns[col].unset(flag);
            self.free_number_boxes[Board::compute_box_index(row, col)].unset(flag);
        }
    }

    pub fn clear_value(&mut self, row: usize, col: usize) {
        let val = self.cells[row][col] as u16;
        if val != 0 {
            let flag = FreeNumberFlags::from(val);
            self.free_number_rows[row].set(flag);
            self.free_number_columns[col].set(flag);
            self.free_number_boxes[Board::compute_box_index(row, col)].set(flag);
            self.cells[row][col] = 0;
        }
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

    fn compute_box_index(row: usize, col: usize) -> usize {
        (row / BOARD_BOX_SIZE) * BOARD_BOX_SIZE + (col / BOARD_BOX_SIZE)
    }
}

#[cfg(test)]
mod tests {

    use crate::board::{Board, FreeNumberFlags};

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
}
