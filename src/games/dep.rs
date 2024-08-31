// mod puzzle;
// mod minesweeper;
// mod grid;
// pub use minesweeper::MineSweeperPlugin;

use bevy::prelude::*;
use rand::Rng;
// use rand::SeedableRng;
// use rand_chacha::ChaCha8Rng;

const GRID_SIZE: usize = 10;
const MINE_COUNT: usize = 15;
const CELL_SIZE: f32 = 50.0;

pub struct MSPlugin;
impl Plugin for MSPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameState>()
            .add_systems(Startup, setup)
            .add_systems(Update, (handle_click, update_text))
            .observe(reveal_cell)
            .observe(check_win_condition);
    }
}

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
    is_mine: bool,
    is_revealed: bool,
    adjacent_mines: u8,
}

#[derive(Event)]
struct RevealCell {
    entity: Entity,
}

#[derive(Event)]
struct CheckWinCondition;

#[derive(Resource)]
struct GameState {
    grid: Vec<Vec<Entity>>,
    game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            grid: Vec::new(),
            game_over: false,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game_state: ResMut<GameState>) {
    commands.spawn(Camera2dBundle::default());

    
    let mut rng = rand::thread_rng();
    let mines: Vec<(usize, usize)> = (0..MINE_COUNT)
        .map(|_| (rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE)))
        .collect();

    let root = commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(25.0)),
                align_self: AlignSelf::Stretch,
                justify_self: JustifySelf::Stretch,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                align_content: AlignContent::FlexStart,
                ..Default::default()
            },
            background_color: Color::srgb(0.25, 0.25, 0.25).into(),
            ..Default::default()
        })
        .id();

    let game_board_node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(50.),
                    height: Val::Px(50.),
                    // border,
                    margin: UiRect::all(Val::Px(20.)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                // background_color: Color::srgb(0.5, 0.0, 0.0),
                // border_color: Color::srgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            // Outline {
            //     width: Val::Px(6.),
            //     offset: Val::Px(6.),
            //     color: Color::WHITE,
            // },
        ))
        .id();

    for y in 0..GRID_SIZE {
        let mut row = Vec::new();
        for x in 0..GRID_SIZE {
            let is_mine = mines.contains(&(x, y));
            let adjacent_mines = count_adjacent_mines(x, y, &mines);
            let cell = Cell {
                x,
                y,
                is_mine,
                is_revealed: false,
                adjacent_mines,
            };

            let entity = commands
                .spawn((
                    cell,
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.5, 0.5, 0.5),
                            custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            x as f32 * CELL_SIZE - (GRID_SIZE as f32 * CELL_SIZE / 2.0),
                            y as f32 * CELL_SIZE - (GRID_SIZE as f32 * CELL_SIZE / 2.0),
                            0.0,
                        ),
                        ..default()
                    },
                ))
                .id();

            row.push(entity);
            commands.entity(game_board_node).add_child(entity);
        }
        game_state.grid.push(row);
    }

    // let text_node = commands.spawn(
    //     Text2dBundle {
    //         text: Text::from_section("Minesweeper", TextStyle {
    //             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //             font_size: 60.0,
    //             ..default()
    //         })
    //             .with_justify(JustifyText::Center),
    //         ..default()
    //     }.with_style(Style {
    //         position_type: PositionType::Absolute,
    //         bottom: Val::Px(5.0),
    //         right: Val::Px(5.0),
    //         ..default()
    //     }),
    // ).id();
    let label_node = commands
        .spawn(TextBundle::from_section(
            "MineSweeper",
            TextStyle {
                font_size: 9.0,
                ..Default::default()
            },
        ))
        .id();

    let container = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .push_children(&[game_board_node, label_node])
        .id();
    commands.entity(root).add_child(container);
}

fn count_adjacent_mines(x: usize, y: usize, mines: &[(usize, usize)]) -> u8 {
    let mut count = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
                if mines.contains(&(nx as usize, ny as usize)) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    game_state: Res<GameState>,
    mut commands: Commands,
) {
    if game_state.game_over {
        return;
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_q.single();
        let window = windows.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let grid_x = (world_position.x + (GRID_SIZE as f32 * CELL_SIZE / 2.0)) / CELL_SIZE;
            let grid_y = (world_position.y + (GRID_SIZE as f32 * CELL_SIZE / 2.0)) / CELL_SIZE;

            if grid_x >= 0.0
                && grid_x < GRID_SIZE as f32
                && grid_y >= 0.0
                && grid_y < GRID_SIZE as f32
            {
                let x = grid_x as usize;
                let y = grid_y as usize;
                let entity = game_state.grid[y][x];
                commands.trigger_targets(RevealCell { entity }, entity);
            }
        }
    }
}

fn reveal_cell(
    trigger: Trigger<RevealCell>,
    mut cell_query: Query<(&mut Cell, &mut Sprite)>,
    game_state: Res<GameState>,
    mut commands: Commands,
) {
    let entity = trigger.event().entity;
    let (mut cell, mut sprite) = cell_query.get_mut(entity).unwrap();

    if cell.is_revealed {
        return;
    }

    cell.is_revealed = true;

    if cell.is_mine {
        sprite.color = Color::srgb(1.0, 0.0, 0.0);
        commands.insert_resource(GameState {
            grid: game_state.grid.clone(),
            game_over: true,
        });
    } else {
        sprite.color = Color::WHITE;
        if cell.adjacent_mines == 0 {
            // Reveal adjacent cells
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = cell.x as i32 + dx;
                    let ny = cell.y as i32 + dy;
                    if nx >= 0 && nx < GRID_SIZE as i32 && ny >= 0 && ny < GRID_SIZE as i32 {
                        let neighbor_entity = game_state.grid[ny as usize][nx as usize];
                        commands.trigger_targets(RevealCell { entity: neighbor_entity }, neighbor_entity);
                    }
                }
            }
        }
    }

    commands.trigger(CheckWinCondition);
}

fn check_win_condition(
    _trigger: Trigger<CheckWinCondition>,
    cell_query: Query<&Cell>,
    game_state: Res<GameState>,
    mut commands: Commands,
) {
    if game_state.game_over {
        return;
    }

    let all_non_mine_cells_revealed = cell_query
        .iter()
        .filter(|cell| !cell.is_mine)
        .all(|cell| cell.is_revealed);

    if all_non_mine_cells_revealed {
        commands.insert_resource(GameState {
            grid: game_state.grid.clone(),
            game_over: true,
        });
    }
}

fn update_text(
    game_state: Res<GameState>,
    mut text_query: Query<&mut Text>,
    cell_query: Query<&Cell>,
) {
    let mut text = text_query.get_single_mut().unwrap();
    let revealed_count = cell_query.iter().filter(|cell| cell.is_revealed).count();
    let total_cells = GRID_SIZE * GRID_SIZE;

    if game_state.game_over {
        let all_non_mine_cells_revealed = cell_query
            .iter()
            .filter(|cell| !cell.is_mine)
            .all(|cell| cell.is_revealed);

        if all_non_mine_cells_revealed {
            text.sections[0].value = "You win!".to_string();
        } else {
            text.sections[0].value = "Game Over!".to_string();
        }
    } else {
        text.sections[0].value = format!("Revealed: {}/{}", revealed_count, total_cells);
    }
}