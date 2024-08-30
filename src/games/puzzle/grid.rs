use super::*;

#[derive(Resource, Default)]
pub struct Grid<C: Cell + Default> {
    pub cells: Vec<Vec<C>>,
    pub rows: usize,
    pub cols: usize,
}

impl<C: Cell> Grid<C> {
    pub fn new(rows: usize, cols: usize, cells: Vec<Vec<C>>) -> Self {
        assert_eq!(cells.len(), rows);
        assert!(cells.iter().all(|row| row.len() == cols));
        Grid { cells, rows, cols }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&C> {
        self.grid.get(row).and_then(|r| r.get(col))
    }

    pub fn get_cell_mut(&mut self, row: usize, col: usize) -> Option<&mut C> {
        self.grid.get_mut(row).and_then(|r| r.get_mut(col))
    }

    pub fn search_cell(&self, from: &C, direction: Direction, distance: usize, edge_behavior: EdgeBehavior) -> Option<&C> {
        let (mut row, mut col) = from.get_pos();
        for _ in 0..distance {
            match direction {
                Direction::Left => col = self.wrap_or_stop(col, self.cols, -1, edge_behavior)?,
                Direction::Right => col = self.wrap_or_stop(col, self.cols, 1, edge_behavior)?,
                Direction::Up => row = self.wrap_or_stop(row, self.rows, -1, edge_behavior)?,
                Direction::Down => row = self.wrap_or_stop(row, self.rows, 1, edge_behavior)?,
                Direction::LeftUp => {
                    col = self.wrap_or_stop(col, self.cols, -1, edge_behavior)?;
                    row = self.wrap_or_stop(row, self.rows, -1, edge_behavior)?;
                },
                Direction::LeftDown => {
                    col = self.wrap_or_stop(col, self.cols, -1, edge_behavior)?;
                    row = self.wrap_or_stop(row, self.rows, 1, edge_behavior)?;
                },
                Direction::RightUp => {
                    col = self.wrap_or_stop(col, self.cols, 1, edge_behavior)?;
                    row = self.wrap_or_stop(row, self.rows, -1, edge_behavior)?;
                },
                Direction::RightDown => {
                    col = self.wrap_or_stop(col, self.cols, 1, edge_behavior)?;
                    row = self.wrap_or_stop(row, self.rows, 1, edge_behavior)?;
                },
            }
        }
        self.get_cell(row, col)
    }

    pub fn wrap_or_stop(&self, current: usize, max: usize, delta: i32, edge_behavior: EdgeBehavior) -> Option<usize> {
        match edge_behavior {
            EdgeBehavior::Stop => {
                if delta < 0 && current == 0 || delta > 0 && current == max - 1 {
                    None
                } else {
                    Some((current as i32 + delta).rem_euclid(max as i32) as usize)
                }
            },
            EdgeBehavior::Wrap => Some((current as i32 + delta).rem_euclid(max as i32) as usize),
        }
    }

    pub fn search_cells(&self, from: &C, position_rule: &PositionRule) -> Vec<&C> {
        let mut results = Vec::new();
        let directions = [
            Direction::Left, Direction::Right, Direction::Up, Direction::Down,
            Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown
        ];

        for &direction in &directions {
            if position_rule.directions.contains(direction) {
                match position_rule.distance {
                    Distance::Finite(dist) => {
                        if let Some(cell) = self.search_cell(from, direction, dist, position_rule.edge_behavior) {
                            results.push(cell);
                        }
                    },
                    Distance::Infinite => {
                        let mut dist = 1;
                        while let Some(cell) = self.search_cell(from, direction, dist, position_rule.edge_behavior) {
                            results.push(cell);
                            dist += 1;
                            if position_rule.edge_behavior == EdgeBehavior::Wrap && dist > self.rows.max(self.cols) {
                                break; // Prevent infinite loop in wrap-around boards
                            }
                        }
                    },
                }
            }
        }
        results
    }

    pub fn is_neighbor(&self, rule: &PositionRule, cell_a: &C, cell_b: &C) -> bool {
        let cells = self.search_cells(cell_a, rule);
        cells.iter().any(|&cell| cell.get_pos() == cell_b.get_pos())
    }

    // pub fn is_orthogonally_continuous(&self, cell_a: &C, cell_b: &C) -> bool {
    //     let (row_a, col_a) = cell_a.get_pos();
    //     let (row_b, col_b) = cell_b.get_pos();
    //     row_a == row_b || col_a == col_b
    // } // 그래프 탐색 기반으로 다시 짜야

    // pub fn is_diagonally_continuous(&self, cell_a: &C, cell_b: &C) -> bool {
    //     let (row_a, col_a) = cell_a.get_pos();
    //     let (row_b, col_b) = cell_b.get_pos();
    //     (row_a as i32 - row_b as i32).abs() == (col_a as i32 - col_b as i32).abs()
    // } // 그래프 탐색 기반으로 다시 짜야

    pub fn count_occurrences<F>(&self, cell_state: &C::CellState, rule: &PositionRule, predicate: F) -> usize
    where
        F: Fn(&C::CellState) -> bool,
    {
        self.search_cells(cell_state, rule)
            .iter()
            .filter(|cell| predicate(cell.get_state()))
            .count()
    }

    pub fn occur(&self, variant: C::CellVariant, number: usize, rule: &PositionRule) -> bool {
        let mut count = 0;
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(cell) = self.get_cell(row, col) {
                    let cells = self.search_cells(cell, rule);
                    count += cells.iter().filter(|&&c| c.get_variant() == variant).count();
                }
            }
        }
        count == number
    }

    pub fn is_pair_exist(&self, variant_a: C::CellVariant, number_a: usize, 
                     variant_b: C::CellVariant, number_b: usize, rule: &PositionRule) -> bool {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if let Some(cell) = self.get_cell(row, col) {
                    let cells = self.search_cells(cell, rule);
                    let count_a = cells.iter().filter(|&&c| c.get_variant() == variant_a).count();
                    let count_b = cells.iter().filter(|&&c| c.get_variant() == variant_b).count();
                    if count_a == number_a && count_b == number_b {
                        return true;
                    }
                }
            }
        }
        false
    } // 보강 필요함.

    // Other helper methods...
}