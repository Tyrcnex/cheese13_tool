use std::ops::{Not, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)] // no copy!!!!!!!!!!
pub struct Bitboard {
    pub cols: [u64; 10]
}

impl Bitboard {
    #[inline(always)]
    pub fn fold_and(&self) -> u64 {
        self.cols.iter().fold(!0, |a, b| a & b)
    }

    #[inline(always)]
    pub fn fold_or(&self) -> u64 {
        self.cols.iter().fold(0, |a, b| a | b)
    }

    #[inline(always)]
    pub fn fold_xor(&self) -> u64 {
        self.cols.iter().fold(0, |a, b| a ^ b)
    }
}

macro_rules! impl_op {
    ($func_name:ident, $op:tt) => {
        type Output = Self;

        #[inline(always)]
        fn $func_name(self, rhs: Self) -> Self {
            Self { cols: std::array::from_fn(|x| self.cols[x] $op rhs.cols[x]) }
        }
    };
}

impl Not for Bitboard {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self {
        Self { cols: self.cols.map(|x| !x) }
    }
}

impl BitAnd for Bitboard { impl_op!(bitand, &); }
impl BitOr for Bitboard { impl_op!(bitor, |); }
impl BitXor for Bitboard { impl_op!(bitxor, ^); }

macro_rules! impl_op_assign {
    ($func_name:ident, $op:tt) => {
        #[inline(always)]
        fn $func_name(&mut self, rhs: Self) {
            for x in 0..10 {
                self.cols[x] $op rhs.cols[x];
            }
        }
    };
}

impl BitAndAssign for Bitboard { impl_op_assign!(bitand_assign, &=); }
impl BitOrAssign for Bitboard { impl_op_assign!(bitor_assign, |=); }
impl BitXorAssign for Bitboard { impl_op_assign!(bitxor_assign, ^=); }