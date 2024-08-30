// use bevy::{a11y::accesskit::Point, prelude::*};

// trait Cell {
//     type CellState;
//     type CellSolution;
//     fn render<T: Bundle>(&self, grid: Grid<C>) -> T;
// }

// #[derive(Resource)]
// struct Grid<C: Cell> {
//     grid: Vec<Vec<C>>,
//     row: usize,
//     col: usize,
// }

// struct MineSweeperCell {
//     solution: MineSweeperCellSolution,
//     state: MineSweeperCellState,
//     pos: (usize, usize),
// }
// enum MineSweeperCellSolution {
//     Mine,
//     AdjacentMine(usize),
// }
// enum MineSweeperCellState {
//     Revealed(Entity),
//     Flagged,
//     Unrevealed,
// }
// impl Cell for MineSweeperCell {
//     type CellState = MineSweeperCellState;
//     type CellSolution = MineSweeperCellSolution;
//     fn render<T: Bundle>(&self) -> T {
//         match self.state {
//             Self::CellState::Revealed(entity) => match self.solution {
//                 Self::CellSolution::Mine => {
                    
//                 },
//                 Self::CellSolution::AdjacentMine(adjacent_mine) => {

//                 },
//             },
//             Self::CellState::Flagged => {

//             },
//             Self::CellState::Unrevealed => {
                
//             },
//         }
//     }
// }

// struct GridPlugin {

// }
// impl<C: Cell> Plugin for GridPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .init_resource::<Grid<C>>()
//             .run();
//     }
// }