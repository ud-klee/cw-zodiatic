use bitvec::{prelude::*, slice::IterOnes};

type BV = BitVec<u32, Lsb0>;

#[derive(Clone, Debug, PartialEq)]
pub struct BitMap {
    bv: BV,
}

impl BitMap {
    pub fn new() -> Self {
        Self::from_vec(vec![0])
    }

    pub fn from_vec(vec: Vec<u32>) -> Self {
        BitMap {
            bv: BV::from_vec(vec),
        }
    }

    pub fn ones(len: usize) -> Self {
        BitMap {
            bv: BV::repeat(true, len),
        }
    }

    pub fn into_vec(self) -> Vec<u32> {
        self.bv.into_vec()
    }

    pub fn set(&mut self, index: usize) -> &Self {
        self.bv.set(index, true);
        self
    }

    pub fn and(&mut self, bitmap: BitMap) -> &Self {
        self.bv &= bitmap.bv;
        self
    }

    pub fn or(&mut self, bitmap: BitMap) -> &Self {
        self.bv |= bitmap.bv;
        self
    }

    pub fn xor(&mut self, bitmap: BitMap) -> &Self {
        self.bv ^= bitmap.bv;
        self
    }

    pub fn iter_ones(&self) -> IterOnes<u32, Lsb0> {
        self.bv.iter_ones()
    }
}

impl Default for BitMap {
    fn default() -> Self {
        BitMap::new()
    }
}
