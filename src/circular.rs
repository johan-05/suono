use crate::errors::WrongSizeOfSlice;

use std::error::Error;
use std::ops::Index;

#[derive(Debug)]
pub struct Circular2D<T> {
    buffer: Vec<T>,
    offset: usize,
    dimentions: (usize, usize),
}

impl<T> Circular2D<T>
where
    T: Copy + Default,
{
    pub fn with_dimentions(dim: (usize, usize)) -> Circular2D<T> {
        return Circular2D {
            buffer: vec![T::default(); dim.0 * dim.1],
            offset: 0,
            dimentions: dim,
        };
    }

    pub fn push(&mut self, item: &[T]) -> Result<(), &dyn Error> {
        if item.len() != self.dimentions.1 {
            return Err(Box::leak(Box::new(WrongSizeOfSlice {
                size: self.dimentions.1,
            })));
        }
        let slice_at_idx_offset = &mut self.buffer
            [self.offset * self.dimentions.1..(self.offset + 1) * self.dimentions.1];
        slice_at_idx_offset.copy_from_slice(item);

        self.offset += 1;
        if self.offset == self.dimentions.0 {
            self.offset = 0;
        }
        return Ok(());
    }
}

impl<T> Index<usize> for Circular2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        if index + self.offset >= self.dimentions.0 {
            let indexed_range = self.dimentions.1 * (index + self.offset - self.dimentions.0)
                ..self.dimentions.1 * (index + self.offset - self.dimentions.0 + 1);
            return &self.buffer[indexed_range];
        } else {
            let indexed_range = self.dimentions.1 * (index + self.offset)
                ..self.dimentions.1 * (index + self.offset + 1);
            return &self.buffer[indexed_range];
        }
    }
}
