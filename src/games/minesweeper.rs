// fn reveal_cell(
//     commands: &mut Commands,
//     game_board: Res<GameBoard>,
//     cell_query: Query<(Entity, &Transform, &mut Cell)>,
//     sprite_query: Query<&mut Sprite>,
//     x: usize, y: usize
// ) -> bool {
//     if x >= GRID_SIZE || y >= GRID_SIZE {
//         return false;
//     }

//     let entity = game_board.cells[y][x];
//     if let Ok((_, _, mut cell)) = cell_query.get_mut(entity) {
//         if cell.is_revealed {
//             return false;
//         }

//         cell.is_revealed = true;
//         let mut sprite = sprite_query.get_mut(entity).unwrap();
//         sprite.color = Color::WHITE;

//         match cell.cell_type {
//             CellType::Mine => {
//                 sprite.color = Color::Srgba(Srgba::RED);
//                 return true; // Game over
//             },
//             CellType::Clue(num) if num > 0 => {
//                 commands.entity(entity).with_children(|parent| {
//                     parent.spawn(Text2dBundle {
//                         text: Text::from_section(
//                             num.to_string(),
//                             TextStyle {
//                                 font_size: 20.0,
//                                 color: Color::BLACK,
//                                 ..default()
//                             },
//                         ),
//                         transform: Transform::from_xyz(0.0, 0.0, 1.0),
//                         ..default()
//                     });
//                 });
//             },
//             _ => {
//                 // Reveal adjacent cells
//                 for dy in -1..=1 {
//                     for dx in -1..=1 {
//                         if dx == 0 && dy == 0 {
//                             continue;
//                         }
//                         let nx = x as i32 + dx;
//                         let ny = y as i32 + dy;
//                         if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
//                             reveal_cell(commands, game_board, cell_query, sprite_query, nx as usize, ny as usize);
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     false
// }

use bevy::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
enum CellType {
    Mine,
    Clue(usize),
}
impl Default for CellType {
    fn default() -> Self {
        Self::Clue(0)
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
struct Cell {
    is_revealed: bool,
    cell_type: CellType,
    pos: (usize, usize),
}
impl Cell {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            ..default()
        }
    }
}

#[derive(Debug, Resource)]
struct GameBoard {
    mine_chance: f32,
    grid_size: usize,
    is_generated: bool,
    grid: Vec<Vec<Entity>>,
}

#[derive(Debug, Resource)]
enum GameState {
    GameOver,
    InGame,
    Win,
}
impl GameState {
    fn get(&self) -> &Self {
        self
    }
    fn get_mut(&mut self) -> &mut Self {
        self
    }
}

pub struct MineSweeperPlugin {
    mine_chance: f32,
    grid_size: usize,
}
impl Default for MineSweeperPlugin {
    fn default() -> Self {
        Self {
            mine_chance: 0.15,
            grid_size: 10,
        }
    }
}
impl Plugin for MineSweeperPlugin {
    fn build(&self, app: &mut App) {
        let &Self { mine_chance, grid_size } = self;
        let game_board = GameBoard {
            mine_chance,
            grid_size,
            is_generated: false,
            grid: Vec::new(),
        };
        let game_state = GameState::InGame;
        app
            .insert_resource(game_board)
            .insert_resource(game_state)
            .add_systems(Startup, setup)
            .add_systems(Update, cell_click);
    }
}

fn setup(
    mut commands: Commands,
    game_board: ResMut<GameBoard>,
) {
    commands.spawn(Camera2dBundle::default());

    let GameBoard { grid_size, grid, .. } = game_board.into_inner();
    let grid_size = *grid_size;
    for y in 0..grid_size {
        let mut row = Vec::new();
        for x in 0..grid_size {
            let cell = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.5, 0.5, 0.5),
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 - grid_size as f32 / 2.0) * 32.0,
                            (y as f32 - grid_size as f32 / 2.0) * 32.0,
                            0.0,
                        ),
                        ..default()
                    },
                    Cell::new((y, x)),
                ))
                .id();
            row.push(cell);
        }
        grid.push(row);
    }
}

fn generate_puzzle(
    game_board: ResMut<GameBoard>,
    mut query: Query<&mut Cell>,
    start_x: usize,
    start_y: usize,
) {
    let GameBoard { grid_size, mine_chance, grid, is_generated } = game_board.into_inner();
    let grid_size = *grid_size;
    let mut rng = rand::thread_rng();

    // Place mines
    for y in 0..grid_size {
        for x in 0..grid_size {
            if y == start_y && x == start_x {
                continue; // Ensure the starting cell is not a mine
            }
            let is_mine = rng.gen::<f32>() < *mine_chance;
            if !is_mine { continue }
            let cell_id = grid[y][x];
            if let Ok(ref mut cell) = query.get_mut(cell_id) {
                cell.cell_type = CellType::Mine;
            } else {
                error!("Failed to get cell from Cell query!");
            }
        }
    }

    // Calculate adjacent mines
    for y in 0..grid_size {
        for x in 0..grid_size {
            let cell_id = grid[y][x];
            if let Ok(cell) = query.get(cell_id) {
                if let CellType::Mine = cell.cell_type { continue }
            }
            let mut count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 { continue                     }
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < grid_size as i32 && ny >= 0 && ny < grid_size as i32 {
                        let neighbor_id = grid[ny as usize][nx as usize];
                        if let Ok(neighbor_cell) = query.get(neighbor_id) {
                            if let CellType::Mine = neighbor_cell.cell_type {
                                count += 1;
                            }
                        } else {
                            error!("Failed to get neighbor_cell from Cell query!");
                        }
                    }
                }
            }
            if let Ok(mut cell) = query.get_mut(cell_id) {
                if let CellType::Clue(ref mut num) = cell.cell_type {
                    *num = count;
                }
            } else { unreachable!() }
        }
    }
    *is_generated = true;
}

fn cell_click(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut game_board: Res<GameBoard>,
    mut game_state: ResMut<GameState>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    trans_query: Query<&Transform>,
    mut cell_query: Query<&mut Cell>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let is_game_over = if let GameState::GameOver = *game_state.get() {
        true
    }  else { false };
    if is_game_over { return }

    if mouse_button_input.pressed(MouseButton::Left) {
        let GameBoard { grid_size, grid, is_generated, .. } = game_board.into_inner();
        let grid_size = *grid_size;
        let (camera, camera_transform) = camera_query.single();

        if let Some(world_position) = windows.single().cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()) 
        {
            println!("{world_position:?}");
        //     trans_query.get()
        //         for (entity, transform, cell) in cell_query.iter() {
        //             let cell_pos = transform.translation.truncate();
        //             if world_position.distance(cell_pos) < 15.0 && !cell.is_revealed {
        //                 let grid_x = ((cell_pos.x + (grid_size as f32 / 2.0) * 32.0) / 32.0) as usize;
        //                 let grid_y = ((cell_pos.y + (grid_size as f32 / 2.0) * 32.0) / 32.0) as usize;

        //                 if !is_generated {
        //                     generate_puzzle(game_board, cell_query, grid_x, grid_y);
        //                 }

        //                 // let is_game_over = reveal_cell(&mut commands, game_board, cell_query, sprite_query, grid_x, grid_y);
        //                 if is_game_over {
        //                     let state = game_state.get_mut();
        //                     *state = GameState::GameOver;
        //                     println!("Game Over!");
        //                 }

        //                 break;
        //             }
        //         }
        //     }
        }
    }
}