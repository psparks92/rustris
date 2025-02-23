
#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Empty,
    Occupied { r: u8, g: u8, b: u8 },
}
