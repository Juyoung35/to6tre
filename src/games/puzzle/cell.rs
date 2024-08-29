pub trait Cell {
    type CellVariant;
    fn get_variant(&self) -> &Self::CellVariant;
    fn is_revealed(&self) -> bool;
    fn reveal(&mut self);
    fn hide(&mut self);
}