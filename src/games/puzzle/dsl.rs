use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PuzzleType {
    Sudoku,
    Minesweeper,
    TentsAndTrees,
    Aquarium,
    Nonogram,
    Crossword,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    pub puzzle_type: PuzzleType,
    pub grid: Grid,
    pub regions: Vec<Region>,
    pub rules: Vec<Rule>,
    pub cell_variants: Vec<CellVariant>,
    pub rendering: RenderingInfo,
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub value: Option<CellContent>,
    pub region_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CellContent {
    Number(u8),
    Symbol(char),
    Custom(String),
    Water,
    Empty,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub id: usize,
    pub cells: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub enum Rule {
    UniqueInRow(usize),
    UniqueInColumn(usize),
    UniqueInRegion(usize),
    SumEquals(Vec<(usize, usize)>, i32),
    AdjacentMines(usize, usize, u8),
    TotalMines(u32),
    TentsInRow(usize, u8),
    TentsInColumn(usize, u8),
    TentTreePairing,
    NoAdjacentTents,
    WaterInRow(usize, u8),
    WaterInColumn(usize, u8),
    WaterFlowsUpward,
}

#[derive(Debug, Clone)]
pub struct CellVariant {
    pub content: CellContent,
    pub proportion: f32,
    pub rendering: CellRenderingInfo,
}

#[derive(Debug, Clone)]
pub struct RenderingInfo {
    pub cell_size: f32,
    pub grid_color: Color,
    pub background_color: Color,
    pub region_border_color: Color,
}

#[derive(Debug, Clone)]
pub enum CellRenderingInfo {
    Sprite(SpriteBundle),
    Mesh2D(Mesh2dBundle),
    Text(TextBundle),
}

#[derive(Debug, Clone)]
pub struct PuzzleDefinition {
    pub puzzle: Puzzle,
    pub initial_state: Vec<CellState>,
}

#[derive(Debug, Clone)]
pub struct CellState {
    pub x: usize,
    pub y: usize,
    pub content: CellContent,
    pub revealed: bool,
    pub flagged: bool,
}

// Helper functions (create_sprite_bundle, create_mesh2d_bundle, create_text_bundle) remain the same