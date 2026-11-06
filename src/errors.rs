use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct WrongSizeOfSlice {
    pub size: usize,
}

impl std::fmt::Display for WrongSizeOfSlice {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        println!("Wring Size of Slice: Slice must have length {}", self.size);
        Ok(())
    }
}
impl Error for WrongSizeOfSlice {}
