use crate::constraint;

pub struct Cell {
    pub data: CellData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CellData {
    Text(String),
    Null,
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Cell {
            data: self.data.clone(),
        }
    }
}

impl Cell {
    pub fn new(data: CellData) -> Self {
        Cell { data: data }
    }
}
