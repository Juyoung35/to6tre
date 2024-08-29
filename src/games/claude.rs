use bevy::{a11y::accesskit::Point, prelude::*};

trait Cell {
    type CellState;
    type CellSolution;
    fn render<T: Bundle>(&self, grid: &Grid<Self>) -> T where Self: Sized;
}

#[derive(Resource)]
struct Grid<C: Cell> {
    grid: Vec<Vec<C>>,
    row: usize,
    col: usize,
}

struct MineSweeperCell {
    solution: MineSweeperCellSolution,
    state: MineSweeperCellState,
    pos: (usize, usize),
}

enum MineSweeperCellSolution {
    Mine,
    AdjacentMine(usize),
}

enum MineSweeperCellState {
    Revealed(Entity),
    Flagged,
    Unrevealed,
}

impl Cell for MineSweeperCell {
    type CellState = MineSweeperCellState;
    type CellSolution = MineSweeperCellSolution;
    fn render<T: Bundle>(&self, grid: &Grid<Self>) -> T {
        match self.state {
            Self::CellState::Revealed(entity) => match self.solution {
                Self::CellSolution::Mine => {
                    // Implement mine rendering
                    todo!()
                },
                Self::CellSolution::AdjacentMine(adjacent_mine) => {
                    // Implement adjacent mine rendering
                    todo!()
                },
            },
            Self::CellState::Flagged => {
                // Implement flagged cell rendering
                todo!()
            },
            Self::CellState::Unrevealed => {
                // Implement unrevealed cell rendering
                todo!()
            },
        }
    }
}

struct GridPlugin<C: Cell> {
    phantom: std::marker::PhantomData<C>,
}

impl<C: Cell> Plugin for GridPlugin<C> {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Grid<C>>()
            .add_systems(Update, update_grid::<C>);
    }
}

fn update_grid<C: Cell>(mut grid: ResMut<Grid<C>>) {
    // Implement grid update logic
    todo!()
}