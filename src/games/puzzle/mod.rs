mod cell;
mod direction;
mod grid;
mod position_rule;

use bevy::prelude::*;
use rand::prelude::*;
use std::marker::PhantomData;
use std::hash::Hash;

use cell::Cell;
use grid::Grid;

pub struct PuzzleRule<C: Cell> {
    pub description: String,
    pub condition: Box<dyn Fn(&Grid<C>) -> bool>,
}

pub struct PuzzleDefinition<C: Cell> {
    pub rows: usize,
    pub cols: usize,
    pub rules: Vec<PuzzleRule<C>>,
}

impl<C: Cell> PuzzleDefinition<C> {
    pub fn validate(&self, grid: &Grid<C>) -> bool {
        self.rules.iter().all(|rule| (rule.condition)(grid))
    }

    pub fn generate_puzzle(&self, seed: u64) -> Grid<C> {
        let mut rng = StdRng::seed_from_u64(seed);
        // Initialize an empty grid
        let mut grid = Grid {
            cells: vec![vec![C::default(); cols]; rows],
            rows,
            cols,
        };
        self.generate_full_solution(&mut grid, &mut rng);
        self.reduce_revealed_cells(&mut grid, &mut rng);
        grid
    }

    fn generate_full_solution<C: Cell>(&self, grid: &mut Grid<C>, rng: &mut StdRng)
    where
        C::CellState: Clone + Default + Eq + Hash,
    {
        // Implement backtracking algorithm to generate a full solution
        // This is a simplified version and may need to be adapted for specific puzzle types
        fn backtrack<C: Cell>(grid: &mut Grid<C>, rng: &mut StdRng, pos: usize) -> bool
        where
            C::CellState: Clone + Default + Eq + Hash,
        {
            if pos == grid.rows * grid.cols {
                return true;
            }

            let row = pos / grid.cols;
            let col = pos % grid.cols;

            let mut states: Vec<C::CellState> = vec![C::CellState::default()]; // Add all possible states
            states.shuffle(rng);

            for state in states {
                if let Some(cell) = grid.get_cell_mut(row, col) {
                    cell.set_state(state.clone());
                    if self.validate(grid) && backtrack(grid, rng, pos + 1) {
                        return true;
                    }
                }
            }

            if let Some(cell) = grid.get_cell_mut(row, col) {
                cell.set_state(C::CellState::default());
            }
            false
        }

        backtrack(grid, rng, 0);
    }

    fn reduce_revealed_cells<C: Cell>(&self, grid: &mut Grid<C>, rng: &mut StdRng)
    where
        C::CellState: Clone + Default + Eq + Hash,
    {
        let mut cells: Vec<(usize, usize)> = (0..grid.rows)
            .flat_map(|r| (0..grid.cols).map(move |c| (r, c)))
            .collect();
        cells.shuffle(rng);

        for (row, col) in cells {
            if let Some(cell) = grid.get_cell_mut(row, col) {
                let original_state = cell.get_state().clone();
                cell.set_revealed(false);
                cell.set_state(C::CellState::default());

                if !Self::has_unique_solution(grid) {
                    cell.set_revealed(true);
                    cell.set_state(original_state);
                }
            }
        }
    }

    fn has_unique_solution<C: Cell>(&self, grid: &Grid<C>) -> bool
    where
        C::CellState: Clone + Default + Eq + Hash,
    {
        // Implement a solver to check if the puzzle has a unique solution
        // This is a simplified version and may need to be adapted for specific puzzle types
        fn solve<C: Cell>(grid: &mut Grid<C>, pos: usize) -> bool
        where
            C::CellState: Clone + Default + Eq + Hash,
        {
            if pos == grid.rows * grid.cols {
                return true;
            }

            let row = pos / grid.cols;
            let col = pos % grid.cols;

            if let Some(cell) = grid.get_cell(row, col) {
                if cell.is_revealed() {
                    return solve(grid, pos + 1);
                }
            }

            let mut solutions = 0;
            let states: Vec<C::CellState> = vec![C::CellState::default()]; // Add all possible states

            for state in states {
                if let Some(cell) = grid.get_cell_mut(row, col) {
                    cell.set_state(state.clone());
                    if self.validate(grid) && solve(grid, pos + 1) {
                        solutions += 1;
                        if solutions > 1 {
                            return false;
                        }
                    }
                }
            }

            if let Some(cell) = grid.get_cell_mut(row, col) {
                cell.set_state(C::CellState::default());
            }
            solutions == 1
        }

        let mut grid_copy = grid.clone();
        solve(&mut grid_copy, 0)
    }
}

#[macro_export]
macro_rules! define_puzzle_rules {
    ($name:ident {
        $($rule_name:ident($cell:ident: &$cell_type:ty) => $rule_body:expr),*$(,)?
    }) => {
        struct $name;
        impl $name {
            $(
                fn $rule_name($cell: &$cell_type, grid: &Grid<$cell_type>) -> bool {
                    $rule_body
                }
            )*
        }
    };
}

// // Example usage of the macro
// define_puzzle_rules! {
//     SudokuRules {
//         no_duplicates_in_row(row: usize) => {
//             let mut seen = HashSet::new();
//             for col in 0..grid.cols {
//                 if let Some(cell) = grid.get_cell(row, col) {
//                     if !seen.insert(cell.get_state()) {
//                         return false;
//                     }
//                 }
//             }
//             true
//         },
//         no_duplicates_in_column(col: usize) => {
//             let mut seen = HashSet::new();
//             for row in 0..grid.rows {
//                 if let Some(cell) = grid.get_cell(row, col) {
//                     if !seen.insert(cell.get_state()) {
//                         return false;
//                     }
//                 }
//             }
//             true
//         },
//         no_duplicates_in_3x3(start_row: usize, start_col: usize) => {
//             let mut seen = HashSet::new();
//             for row in start_row..start_row + 3 {
//                 for col in start_col..start_col + 3 {
//                     if let Some(cell) = grid.get_cell(row, col) {
//                         if !seen.insert(cell.get_state()) {
//                             return false;
//                         }
//                     }
//                 }
//             }
//             true
//         },
//     }
// }

// // Example of how to use the defined rules
// fn check_sudoku<C: Cell>(grid: &Grid<C>) -> bool {
//     // Check rows and columns
//     for i in 0..9 {
//         if !SudokuRules::no_duplicates_in_row(i, grid) || !SudokuRules::no_duplicates_in_column(i, grid) {
//             return false;
//         }
//     }

//     // Check 3x3 sub-grids
//     for i in (0..9).step_by(3) {
//         for j in (0..9).step_by(3) {
//             if !SudokuRules::no_duplicates_in_3x3(i, j, grid) {
//                 return false;
//             }
//         }
//     }

//     true
// }

pub struct GridPlugin<C: Cell + Component> {
    _phantom: PhantomData<C>,
}

impl<C: Cell + Component> Plugin for GridPlugin<C> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<C>>()
           .add_systems(Update, update_grid::<C>);
    }
}

fn update_grid<C: Cell + Component>(grid: ResMut<Grid<C>>) {
    // Implementation of grid update logic...
}

// Example usage:
// define_puzzle_rules! {
//     SudokuRules {
//         check_row(cell: &SudokuCell) => {
//             // Implementation of row checking logic...
//         },
//         check_column(cell: &SudokuCell) => {
//             // Implementation of column checking logic...
//         },
//         // More rules...
//     }
// }
//
// let sudoku_puzzle = PuzzleDefinition {
//     rows: 9,
//     cols: 9,
//     rules: vec![
//         PuzzleRule {
//             description: "Each row must contain digits 1-9 without repetition".to_string(),
//             condition: Box::new(|grid| SudokuRules::check_row(grid)),
//         },
//         // More rules...
//     ],
// };
//
// let generated_puzzle = sudoku_puzzle.generate_puzzle(12345);
// let is_valid = sudoku_puzzle.validate(&generated_puzzle);