// use bevy::prelude::*;
// use bevy::input::mouse::MouseButtonInput;
// use rand::Rng;



// fn setup(
//     mut commands: Commands,
//     mut game_board: ResMut<GameBoard>
// ) {
//     commands.spawn(Camera2dBundle::default());

//     for y in 0..GRID_SIZE {
//         let mut row = Vec::new();
//         for x in 0..GRID_SIZE {
//             let cell = commands
//                 .spawn((
//                     SpriteBundle {
//                         sprite: Sprite {
//                             color: Color::Srgba(0.5, 0.5, 0.5, 1.0),
//                             custom_size: Some(Vec2::new(30.0, 30.0)),
//                             ..default()
//                         },
//                         transform: Transform::from_xyz(
//                             (x as f32 - GRID_SIZE as f32 / 2.0) * 32.0,
//                             (y as f32 - GRID_SIZE as f32 / 2.0) * 32.0,
//                             0.0,
//                         ),
//                         ..default()
//                     },
//                     Cell {
//                         is_revealed: false,
//                         cell_type: CellType::Clue(0),
//                     },
//                 ))
//                 .id();
//             row.push(cell);
//         }
//         game_board.cells.push(row);
//     }
// }

// fn generate_puzzle(
//     commands: &mut Commands,
//     game_board: Res<GameBoard>,
//     cell_query: Query<(Entity, &Transform, &mut Cell)>,
//     start_x: usize, start_y: usize) {
//     let mut rng = rand::thread_rng();

//     // Place mines
//     for y in 0..GRID_SIZE {
//         for x in 0..GRID_SIZE {
//             if x == start_x && y == start_y {
//                 continue; // Ensure the starting cell is not a mine
//             }
//             let is_mine = rng.gen::<f32>() < MINE_CHANCE;
//             if !is_mine { continue }
//             if let Some((_, _, cell)) = cell_query.get_mut(game_board.cells[y][x]) {
//                 cell.cell_type = CellType::Mine;
//             }
//         }
//     }

//     // Calculate adjacent mines
//     for y in 0..GRID_SIZE {
//         for x in 0..GRID_SIZE {
//             let mut count = 0;
//             for dy in -1..=1 {
//                 for dx in -1..=1 {
//                     if dx == 0 && dy == 0 {
//                         continue;
//                     }
//                     let nx = x as i32 + dx;
//                     let ny = y as i32 + dy;
//                     if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
//                         if let Some(neighbor) = commands.get_entity(game_board.cells[ny as usize][nx as usize]) {
//                             if let CellType::Mine = cell_query.get(neighbor).unwrap().cell_type {
//                                 count += 1;
//                             }
//                         }
//                     }
//                 }
//             }
//             if let Some((_, _, cell)) = cell_query.get_mut(game_board.cells[y][x]) {
//                 cell.cell_type = CellType::Clue(count);
//             }
//         }
//     }

//     game_board.is_generated = true;
// }

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

// fn cell_click(
//     mut commands: Commands,
//     mut mouse_button_events: EventReader<MouseButtonInput>,
//     mut game_board: Res<GameBoard>,
//     mut game_state: ResMut<GameState>,
//     camera_query: Query<(&Camera, &GlobalTransform)>,
//     cell_query: Query<(Entity, &Transform, &mut Cell)>,
//     sprite_query: Query<&mut Sprite>,
// ) {
//     if game_state.is_game_over {
//         return;
//     }

//     if mouse_button.just_pressed(MouseButton::Left) {
//         let (camera, camera_transform) = camera_query.single();
//         let window = mouse_butto_events.read().window.get_primary().unwrap();

//         if let Some(world_position) = window.cursor_position()
//             .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//             .map(|ray| ray.origin.truncate())
//         {
//             for (entity, transform, cell) in cell_query.iter() {
//                 let cell_pos = transform.translation.truncate();
//                 if world_position.distance(cell_pos) < 15.0 && !cell.is_revealed {
//                     let grid_x = ((cell_pos.x + (GRID_SIZE as f32 / 2.0) * 32.0) / 32.0) as usize;
//                     let grid_y = ((cell_pos.y + (GRID_SIZE as f32 / 2.0) * 32.0) / 32.0) as usize;

//                     if !game_board.is_generated {
//                         generate_puzzle(&mut commands, game_board, cell_query, grid_x, grid_y);
//                     }

//                     let is_game_over = reveal_cell(&mut commands, game_board, cell_query, sprite_query, grid_x, grid_y);
//                     if is_game_over {
//                         game_state.is_game_over = true;
//                         println!("Game Over!");
//                     }

//                     break;
//                 }
//             }
//         }
//     }
// }
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
enum CellType {
    Mine,
    #[default]
    Clue(usize),
}

#[derive(Component, Clone, Copy, Debug, Default)]
struct Cell {
    is_revealed: bool,
    cell_type: CellType,
}

#[derive(Debug)]
struct GameBoard {
    mine_chance: f32,
    grid_size: usize,
    is_generated: bool,
    grid: Vec<Vec<Entity>>,
}

#[derive(Debug)]
enum GameState {
    GameOver,
    InGame,
    Win,
}

struct MineSweeperPlugin {
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
            // .add_systems(Update, cell_click);
    }
}

fn setup(
    mut commands: Commands,
    mut game_board: ResMut<GameBoard>,
) {
    commands.spawn(Camera2dBundle::default());

    let GameBoard { grid_size, grid, .. } = game_board;
    for y in 0..grid_size {
        let mut row = Vec::new();
        for x in 0..grid_size {
            let cell = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::Srgba(0.5, 0.5, 0.5, 1.0),
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 - GRID_SIZE as f32 / 2.0) * 32.0,
                            (y as f32 - GRID_SIZE as f32 / 2.0) * 32.0,
                            0.0,
                        ),
                        ..default()
                    },
                    Cell {
                        is_revealed: false,
                        cell_type: CellType::Clue(0),
                    },
                ))
                .id();
            row.push(cell);
        }
        grid.push(row);
    }
}