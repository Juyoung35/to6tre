pub trait Cell {
    type CellVariant;
    type CellState;
    fn get_variant(&self) -> &Self::CellVariant;
    fn get_state(&self) -> &Self::CellState;
    fn is_revealed(&self) -> bool;
    fn reveal(&mut self);
    fn hide(&mut self);
}