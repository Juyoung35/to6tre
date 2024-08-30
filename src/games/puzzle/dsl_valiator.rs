use std::collections::HashSet;

// Assuming we have these types defined
use crate::{Rule, Grid, Cell, CellContent, Region};

fn validate_rule(rule: &Rule, grid: &Grid, regions: &[Region]) -> bool {
    match rule {
        Rule::UniqueInRow(row) => validate_unique_in_row(*row, grid),
        Rule::UniqueInColumn(col) => validate_unique_in_column(*col, grid),
        Rule::UniqueInRegion(region_id) => validate_unique_in_region(*region_id, grid, regions),
        Rule::SumEquals(cells, target_sum) => validate_sum_equals(cells, *target_sum, grid),
        Rule::AdjacentMines(x, y, count) => validate_adjacent_mines(*x, *y, *count, grid),
        Rule::TotalMines(count) => validate_total_mines(*count, grid),
        Rule::TentsInRow(row, count) => validate_tents_in_row(*row, *count, grid),
        Rule::TentsInColumn(col, count) => validate_tents_in_column(*col, *count, grid),
        Rule::TentTreePairing => validate_tent_tree_pairing(grid),
        Rule::NoAdjacentTents => validate_no_adjacent_tents(grid),
        Rule::WaterInRow(row, count) => validate_water_in_row(*row, *count, grid),
        Rule::WaterInColumn(col, count) => validate_water_in_column(*col, *count, grid),
        Rule::WaterFlowsUpward => validate_water_flows_upward(grid, regions),
    }
}

fn validate_unique_in_row(row: usize, grid: &Grid) -> bool {
    let values: Vec<&String> = grid.cells
        .iter()
        .filter(|cell| cell.y == row && cell.value.is_some())
        .filter_map(|cell| cell.value.as_ref())
        .collect();
    
    values.len() == values.iter().collect::<HashSet<_>>().len()
}

fn validate_unique_in_column(col: usize, grid: &Grid) -> bool {
    let values: Vec<&String> = grid.cells
        .iter()
        .filter(|cell| cell.x == col && cell.value.is_some())
        .filter_map(|cell| cell.value.as_ref())
        .collect();
    
    values.len() == values.iter().collect::<HashSet<_>>().len()
}

fn validate_unique_in_region(cells: &[(usize, usize)], grid: &Grid) -> bool {
    let values: Vec<&String> = cells.iter()
        .filter_map(|&(x, y)| grid.cells.iter().find(|cell| cell.x == x && cell.y == y))
        .filter_map(|cell| cell.value.as_ref())
        .collect();
    
    values.len() == values.iter().collect::<HashSet<_>>().len()
}

fn validate_sum_equals(cells: &[(usize, usize)], target_sum: i32, grid: &Grid) -> bool {
    let sum: i32 = cells.iter()
        .filter_map(|&(x, y)| grid.cells.iter().find(|cell| cell.x == x && cell.y == y))
        .filter_map(|cell| cell.value.as_ref())
        .filter_map(|value| value.parse::<i32>().ok())
        .sum();
    
    sum == target_sum
}

fn validate_adjacent_mines(x: usize, y: usize, count: u8, grid: &Grid) -> bool {
    let adjacent_cells = [
        (x.wrapping_sub(1), y.wrapping_sub(1)), (x, y.wrapping_sub(1)), (x + 1, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),                                         (x + 1, y),
        (x.wrapping_sub(1), y + 1),             (x, y + 1),             (x + 1, y + 1),
    ];

    let mine_count = adjacent_cells.iter()
        .filter(|&&(ax, ay)| ax < grid.width && ay < grid.height)
        .filter(|&&(ax, ay)| {
            grid.cells.iter()
                .find(|cell| cell.x == ax && cell.y == ay)
                .and_then(|cell| cell.value.as_ref())
                .map_or(false, |content| matches!(content, CellContent::Mine))
        })
        .count();

    mine_count == count as usize
}

fn validate_total_mines(count: u32, grid: &Grid) -> bool {
    let mine_count = grid.cells.iter()
        .filter(|cell| matches!(cell.value, Some(CellContent::Mine)))
        .count();

    mine_count == count as usize
}

fn validate_tents_in_row(row: usize, count: u8, grid: &Grid) -> bool {
    let tent_count = grid.cells.iter()
        .filter(|cell| cell.y == row && matches!(cell.value, Some(CellContent::Tent)))
        .count();
    tent_count == count as usize
}

fn validate_tents_in_column(col: usize, count: u8, grid: &Grid) -> bool {
    let tent_count = grid.cells.iter()
        .filter(|cell| cell.x == col && matches!(cell.value, Some(CellContent::Tent)))
        .count();
    tent_count == count as usize
}

fn validate_tent_tree_pairing(grid: &Grid) -> bool {
    let mut unpaired_tents = grid.cells.iter()
        .filter(|cell| matches!(cell.value, Some(CellContent::Tent)))
        .collect::<Vec<_>>();
    
    let trees = grid.cells.iter()
        .filter(|cell| matches!(cell.value, Some(CellContent::Tree)));
    
    for tree in trees {
        let adjacent_tent = unpaired_tents.iter().position(|&tent| 
            (tent.x as i32 - tree.x as i32).abs() <= 1 && 
            (tent.y as i32 - tree.y as i32).abs() <= 1
        );
        
        if let Some(index) = adjacent_tent {
            unpaired_tents.swap_remove(index);
        }
    }
    
    unpaired_tents.is_empty()
}

fn validate_no_adjacent_tents(grid: &Grid) -> bool {
    for cell in &grid.cells {
        if matches!(cell.value, Some(CellContent::Tent)) {
            let adjacent_cells = [
                (cell.x.wrapping_sub(1), cell.y.wrapping_sub(1)), (cell.x, cell.y.wrapping_sub(1)), (cell.x + 1, cell.y.wrapping_sub(1)),
                (cell.x.wrapping_sub(1), cell.y),                                                   (cell.x + 1, cell.y),
                (cell.x.wrapping_sub(1), cell.y + 1),             (cell.x, cell.y + 1),             (cell.x + 1, cell.y + 1),
            ];
            
            for &(ax, ay) in &adjacent_cells {
                if ax < grid.width && ay < grid.height {
                    if let Some(adjacent_cell) = grid.cells.iter().find(|c| c.x == ax && c.y == ay) {
                        if matches!(adjacent_cell.value, Some(CellContent::Tent)) {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

fn validate_water_in_row(row: usize, count: u8, grid: &Grid) -> bool {
    let water_count = grid.cells.iter()
        .filter(|cell| cell.y == row && matches!(cell.value, Some(CellContent::Water)))
        .count();
    water_count == count as usize
}

fn validate_water_in_column(col: usize, count: u8, grid: &Grid) -> bool {
    let water_count = grid.cells.iter()
        .filter(|cell| cell.x == col && matches!(cell.value, Some(CellContent::Water)))
        .count();
    water_count == count as usize
}

fn validate_water_flows_upward(grid: &Grid, regions: &[Region]) -> bool {
    for region in regions {
        let mut cells = region.cells.iter()
            .map(|&(x, y)| grid.cells.iter().find(|cell| cell.x == x && cell.y == y).unwrap())
            .collect::<Vec<_>>();
        cells.sort_by_key(|cell| cell.y);

        let mut water_started = false;
        for cell in cells {
            match cell.value {
                Some(CellContent::Water) => water_started = true,
                Some(CellContent::Empty) if water_started => return false,
                _ => {}
            }
        }
    }
    true
}