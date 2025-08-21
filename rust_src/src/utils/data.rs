use crate::utils::board::Bitboard;
use serde::{Serialize, Deserialize};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    I, O, T, L, J, S, Z
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rotation {
    North, East, South, West
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Board(pub Bitboard);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PieceLocation {
    pub piece: Piece,
    pub x: i8,
    pub y: i8,
    pub rotation: Rotation,
}

impl Rotation {
    #[inline(always)]
    pub const fn rotate_block(&self, (x,y): (i8, i8)) -> (i8, i8) {
        match self {
            Rotation::North => (x, y),
            Rotation::East => (y, -x),
            Rotation::South => (-x, -y),
            Rotation::West => (-y, x)
        }
    }

    #[inline(always)]
    pub const fn rotate_blocks(&self, blocks: [(i8, i8); 4]) -> [(i8, i8); 4] {
        [
            self.rotate_block(blocks[0]),
            self.rotate_block(blocks[1]),
            self.rotate_block(blocks[2]),
            self.rotate_block(blocks[3])
        ]
    }

    #[inline(always)]
    pub const fn rotate_right(&self) -> Rotation {
        match self {
            Rotation::North => Rotation::East,
            Rotation::East => Rotation::South,
            Rotation::South => Rotation::West,
            Rotation::West => Rotation::North,
        }
    }

    #[inline(always)]
    pub const fn rotate_left(&self) -> Rotation {
        match self {
            Rotation::North => Rotation::West,
            Rotation::West => Rotation::South,
            Rotation::South => Rotation::East,
            Rotation::East => Rotation::North,
        }
    }

    #[inline(always)]
    pub const fn rotate_180(&self) -> Rotation {
        match self {
            Rotation::North => Rotation::South,
            Rotation::East => Rotation::West,
            Rotation::South => Rotation::North,
            Rotation::West => Rotation::East,
        }
    }
}

impl Piece {
    #[inline(always)]
    pub const fn blocks(&self) -> [(i8, i8); 4] {
        match self {
            Piece::Z => [(-1, 1), (0, 1), (0, 0), (1, 0)],
            Piece::S => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            Piece::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            Piece::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            Piece::J => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
            Piece::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
            Piece::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
        }
    }

    #[inline(always)]
    pub const fn from_char(p: char) -> Self {
        match p {
            'i' => Piece::I,
            'o' => Piece::O,
            'j' => Piece::J,
            't' => Piece::T,
            'l' => Piece::L,
            's' => Piece::S,
            'z' => Piece::Z,
            _ => panic!("wtf is this piece")
        }
    }
}

macro_rules! lutify {
    (($e:expr) for $v:ident in [$($val:expr),*]) => {
        [
            $(
                {
                    let $v = $val;
                    $e
                }
            ),*
        ]
    };
}

macro_rules! piece_lut {
    ($v:ident => $e:expr) => {
        lutify!(($e) for $v in [Piece::I, Piece::O, Piece::T, Piece::L, Piece::J, Piece::S, Piece::Z])
    };
}

macro_rules! rotation_lut {
    ($v:ident => $e:expr) => {
        lutify!(($e) for $v in [Rotation::North, Rotation::East, Rotation::South, Rotation::West])
    };
}


impl PieceLocation {
    #[inline(always)]
    pub const fn blocks(&self) -> [(i8, i8); 4] {
        const LUT: [[[(i8, i8); 4]; 4]; 7] =
            piece_lut!(piece => rotation_lut!(rotation => rotation.rotate_blocks(piece.blocks())));
        self.translate_blocks(LUT[self.piece as usize][self.rotation as usize])
    }

    #[inline(always)]
    const fn translate(&self, (x, y): (i8, i8)) -> (i8, i8) {
        (x + self.x, y + self.y)
    }

    #[inline(always)]
    const fn translate_blocks(&self, cells: [(i8, i8); 4]) -> [(i8, i8); 4] {
        [
            self.translate(cells[0]),
            self.translate(cells[1]),
            self.translate(cells[2]),
            self.translate(cells[3]),
        ]
    }

    #[inline(always)]
    pub fn drop(&mut self, heights: &[i8; 10]) {
        // INLINE your four blocks() entries here, instead of calling `blocks()`.
        // Suppose blocks() would return [(x0,y0), (x1,y1), (x2,y2), (x3,y3)].
        let [(x0, y0), (x1, y1), (x2, y2), (x3, y3)] = self.blocks();

        // SAFETY: we know x0..x3 are 0..9 by construction, so unchecked is safe.
        unsafe {
            let h0 = *heights.get_unchecked(x0 as usize);
            let h1 = *heights.get_unchecked(x1 as usize);
            let h2 = *heights.get_unchecked(x2 as usize);
            let h3 = *heights.get_unchecked(x3 as usize);

            let d0 = y0 - h0;
            let d1 = y1 - h1;
            let d2 = y2 - h2;
            let d3 = y3 - h3;

            // unroll the min chain
            let m01 = if d0 < d1 { d0 } else { d1 };
            let m23 = if d2 < d3 { d2 } else { d3 };
            let m  = if m01 < m23 { m01 } else { m23 };

            self.y -= m;
        }
    }

    #[inline(always)]
    pub fn check_garb(&self, height: i8) -> bool {
        let [(_, y0), (_, y1), (_, y2), (_, y3)] = self.blocks();
        y0 < height || y1 < height || y2 < height || y3 < height
    }
}

impl Board {
    pub fn new(arr: [u64; 10]) -> Self {
        Self(Bitboard { cols: arr })
    }

    #[inline(always)]
    pub fn put_piece(&mut self, loc: &PieceLocation) {
        for &(x, y) in &loc.blocks() {
            self.0.cols[x as usize] |= 1 << y;
        }
    }

    #[inline(always)]
    pub fn remove_lines(&mut self) -> u64 {
        let lines = self.0.fold_and();
        for c in &mut self.0.cols {
            clear_lines(c, lines);
        }
        lines
    }

    #[inline(always)]
    pub fn obstructed(&self, loc: &PieceLocation) -> bool {
        for (x, y) in loc.blocks() {
            if x < 0 || x > 9 || y < 0 {
                continue;
            }
            if self.0.cols[x as usize] & (1 << y) > 0 {
                return true;
            }
        }
        false
    }

    #[inline]
    pub fn push_garbage(&mut self, idx: i8) {
        for (x, v) in self.0.cols.iter_mut().enumerate() {
            *v <<= 1;
            if idx != x as i8 {
                *v += 1;
            }
        }
    }

    #[inline]
    pub fn volume_math(&self, x: i8, y: i8) -> i8 {
        let max_height = 64 - self.0.cols[x as usize].leading_zeros() as i8;
        let volume_height = max_height - y;
        let mut count = 0;
        for xi in 0..10 {
            count += volume_height - (self.0.cols[xi] >> y).count_ones() as i8;
        }
        (count + 3) / 4 // this rounds up, since ceil(x/d) = floor((x+d-1)/d)
    }

    #[inline(always)]
    pub fn max_height(&self) -> i8 {
        64 - self.0.fold_or().leading_zeros() as i8
    }
}

fn clear_lines(col: &mut u64, mut lines: u64) {
    while lines != 0 {
        let i = lines.trailing_zeros();
        let mask = (1 << i) - 1;
        *col = *col & mask | *col >> 1 & !mask;
        lines &= !(1 << i);
        lines >>= 1;
    }
}