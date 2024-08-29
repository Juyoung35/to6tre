use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Distance {
    Finite(usize),
    Infinite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeBehavior {
    Stop,
    Wrap,
}

pub struct PositionRule {
    pub directions: DirectionSet,
    pub distance: Distance,
    pub edge_behavior: EdgeBehavior,
}

impl PositionRule {
    pub fn new(directions: DirectionSet, distance: Distance, edge_behavior: EdgeBehavior) -> Self {
        Self { directions, distance, edge_behavior }
    }
}

// Helper functions to create common PositionRules...

// // Example usage and helper functions
// fn get_orthogonal_rule(edge_behavior: EdgeBehavior) -> PositionRule {
//     PositionRule::new(
//         DirectionSet::from_directions(&[Direction::Left, Direction::Right, Direction::Up, Direction::Down]),
//         Distance::Finite(1),
//         edge_behavior
//     )
// }

// fn get_diagonal_rule(edge_behavior: EdgeBehavior) -> PositionRule {
//     PositionRule::new(
//         DirectionSet::from_directions(&[Direction::LeftUp, Direction::LeftDown, Direction::RightUp, Direction::RightDown]),
//         Distance::Finite(1),
//         edge_behavior
//     )
// }

// fn get_row_rule(edge_behavior: EdgeBehavior) -> PositionRule {
//     PositionRule::new(
//         DirectionSet::from_directions(&[Direction::Left, Direction::Right]),
//         Distance::Infinite,
//         edge_behavior
//     )
// }

// fn get_column_rule(edge_behavior: EdgeBehavior) -> PositionRule {
//     PositionRule::new(
//         DirectionSet::from_directions(&[Direction::Up, Direction::Down]),
//         Distance::Infinite,
//         edge_behavior
//     )
// }