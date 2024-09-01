use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use rand::Rng;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq)]
struct UVec2<T>
where T: Clone + Copy + Debug + PartialEq
{
    x: T,
    y: T,
}
impl<T> UVec2<T>
where T: Clone + Copy + Debug + PartialEq
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug)]
enum MSCellState {
    Mine,
    Adjacent(usize),
}
impl Default for MSCellState {
    fn default() -> Self {
        Self::Adjacent(0)
    }
}

#[derive(Clone, Copy, Debug, Component)]
struct MSCell {
    pos: UVec2<usize>,
    cell_state: MSCellState,
    is_revealed: bool,
}
impl MSCell {
    fn new(x: usize, y: usize) -> Self {
        Self {
            pos: UVec2::new(x, y),
            cell_state: MSCellState::Adjacent(0),
            is_revealed: false,
        }
    }
    fn is_mine(&self) -> bool {
        match self.cell_state {
            MSCellState::Mine => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Resource)]
pub enum MSGameState {
    GameOver,
    Progressing,
    GameWin,
}
impl MSGameState {
    fn is_game_over(&self) -> bool {
        if let Self::GameOver = self { true } else { false }
    }
}

#[derive(Clone, Debug, Default, Resource)]
pub struct MSGameBoard {
    pub cells: Vec<Vec<Entity>>,
    pub is_mine_spawned: bool,
    pub grid_size: usize,
    pub mine_count: usize,
}

#[derive(Event)]
struct RevealCell {
    entity: Entity,
}

#[derive(Event)]
struct CheckWinCondition;

#[derive(Default)]
pub struct MineSweeperPlugin;

impl Plugin for MineSweeperPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MSGameState::Progressing)
            .init_resource::<MSGameBoard>()
            .observe(check_win_condition)
            .observe(reveal_cell);
    }
}
fn spawn_mines(
    init_pos: UVec2<usize>,
    mut cell_query: Query<&mut MSCell>,
    game_board: Res<MSGameBoard>,
) {
    let mut rng = rand::thread_rng();
    let grid_size = game_board.grid_size;
    let mine_count = game_board.mine_count;

    let mut spawned_mines = 0;
    while spawned_mines < mine_count {
        let [x, y] = [0; 2].map(|_| rng.gen_range(0..grid_size));
        if x == init_pos.x && y == init_pos.y { continue }
        if let Some(&cell_entity) = game_board.cells.get(y).and_then(|row| row.get(x)) {
            if let Ok(mut cell) = cell_query.get_mut(cell_entity) {
                if let MSCellState::Mine = cell.cell_state { continue }
                cell.cell_state = MSCellState::Mine;
                spawned_mines += 1;
            }
        }
    }

    for y in 0..grid_size {
        for x in 0..grid_size {
            if let Some(cell_entity) = game_board.cells.get(y).and_then(|row| row.get(x)) {
                if let Ok(cell) = cell_query.get(*cell_entity) {
                    if let MSCellState::Mine = cell.cell_state {
                        for ry in y.saturating_sub(1)..=usize::min(y + 1, grid_size - 1) {
                            for rx in x.saturating_sub(1)..=usize::min(x + 1, grid_size - 1) {
                                if ry == y && rx == x { continue }
                                if let MSCellState::Adjacent(ref mut count) = cell_query.get_mut(game_board.cells[ry][rx]).unwrap().cell_state { *count += 1 }
                            }
                        }
                    }
                }
            }   
        }
    }
}

fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_board: ResMut<MSGameBoard>,
) {
    let grid_size = game_board.grid_size;
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());

    // Top-level grid (app frame)
    commands
        .spawn(NodeBundle {
            style: Style {
                // Use the CSS Grid algorithm for laying out this node
                display: Display::Grid,
                // Make node fill the entirety of its parent (in this case the window)
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // Set the grid to have 2 columns with sizes [min-content, minmax(0, 1fr)]
                //   - The first column will size to the size of its contents
                //   - The second column will take up the remaining available space
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                // Set the grid to have 3 rows with sizes [auto, minmax(0, 1fr), 20px]
                //  - The first row will size to the size of its contents
                //  - The second row take up remaining available space (after rows 1 and 3 have both been sized)
                //  - The third row will be exactly 20px high
                grid_template_rows: vec![
                    GridTrack::auto(),
                    GridTrack::flex(1.0),
                    GridTrack::px(20.),
                ],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            // Header
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Make this node span two grid columns so that it takes up the entire top tow
                        grid_column: GridPlacement::span(2),
                        padding: UiRect::all(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), "MineSweeper");
                });

            // Main content grid (auto placed in row 2, column 1)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        // Make the height of the node fill its parent
                        height: Val::Percent(100.0),
                        // Make the grid have a 1:1 aspect ratio meaning it will scale as an exact square
                        // As the height is set explicitly, this means the width will adjust to match the height
                        aspect_ratio: Some(1.0),
                        // Use grid layout for this node
                        display: Display::Grid,
                        // Add 24px of padding around the grid
                        padding: UiRect::all(Val::Px(24.0)),
                        // Set the grid to have 4 columns all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized columns
                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(12.0),
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
                    ..default()
                })
                .with_children(|builder| {
                    // Note there is no need to specify the position for each grid item. Grid items that are
                    // not given an explicit position will be automatically positioned into the next available
                    // grid cell. The order in which this is performed can be controlled using the grid_auto_flow
                    // style property.

                    let cells = &mut game_board.cells;
                    for y in 0..grid_size {
                        cells.push(Vec::new());
                        for x in 0..grid_size {
                            let entity_id = builder
                                .spawn((
                                    MSCell::new(x, y),
                                    NodeBundle {
                                        style: Style {
                                            display: Display::Grid,
                                            padding: UiRect::all(Val::Px(3.0)),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                                        ..default()
                                    },
                                    Outline {
                                        width: Val::Px(6.),
                                        offset: Val::Px(6.),
                                        color: Color::srgb(0.75, 0.75, 0.75),
                                    },
                                ))
                                .with_children(|builder| {
                                    builder.spawn(TextBundle::from_section(
                                        " ",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 5.0,
                                            color: Color::BLACK,
                                        },
                                    ));
                                })
                                // .with_children(|builder| {
                                //     builder.spawn(NodeBundle {
                                //         background_color: BackgroundColor(color.into()),
                                //         ..default()
                                //     });
                                // })
                                .id();
                            cells[y].push(entity_id);
                        }
                    }
                    // item_rect(builder, ORANGE);
                    // item_rect(builder, BISQUE);
                    // item_rect(builder, BLUE);
                    // item_rect(builder, CRIMSON);
                    // item_rect(builder, AQUA);
                    // item_rect(builder, ORANGE_RED);
                    // item_rect(builder, DARK_GREEN);
                    // item_rect(builder, FUCHSIA);
                    // item_rect(builder, TEAL);
                    // item_rect(builder, ALICE_BLUE);
                    // item_rect(builder, CRIMSON);
                    // item_rect(builder, ANTIQUE_WHITE);
                    // item_rect(builder, YELLOW);
                    // item_rect(builder, DEEP_PINK);
                    // item_rect(builder, YELLOW_GREEN);
                    // item_rect(builder, SALMON);
                });

            // Right side bar (auto placed in row 2, column 2)
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        // Align content towards the start (top) in the vertical axis
                        align_items: AlignItems::Start,
                        // Align content towards the center in the horizontal axis
                        justify_items: JustifyItems::Center,
                        // Add 10px padding
                        padding: UiRect::all(Val::Px(10.)),
                        // Add an fr track to take up all the available space at the bottom of the column so that the text nodes
                        // can be top-aligned. Normally you'd use flexbox for this, but this is the CSS Grid example so we're using grid.
                        grid_template_rows: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::fr(1.0)],
                        // Add a 10px gap between rows
                        row_gap: Val::Px(10.),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLACK),
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Sidebar",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                    ));
                    builder.spawn(TextBundle::from_section(
                        "A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely. A paragraph of text which ought to wrap nicely.",
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                    ));
                    builder.spawn(NodeBundle::default());
                });

            // Footer / status bar
            builder.spawn(NodeBundle {
                style: Style {
                    // Make this node span two grid column so that it takes up the entire bottom row
                    grid_column: GridPlacement::span(2),
                    ..default()
                },
                background_color: BackgroundColor(Color::WHITE),
                ..default()
            });
        });

    for y in 0..grid_size {
        for x in 0..grid_size {
            let entity = game_board.cells[y][x];
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.insert(
                    On::<Pointer<Click>>::commands_mut(move |click, commands| {
                        match click.button {
                            PointerButton::Primary => {
                                commands.trigger_targets(RevealCell { entity }, entity);
                            },
                            PointerButton::Secondary => {
                                // TODO: flagging
                            },
                            _ => (),
                        }
                    })
                );
            }
        }
    }
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
    builder.spawn(
        TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: 24.0,
                color: Color::BLACK,
            },
        ),
    );
}

fn check_win_condition(
    _trigger: Trigger<CheckWinCondition>,
    cell_query: Query<&MSCell>,
    mut game_state: ResMut<MSGameState>,
) {
    if game_state.is_game_over() { return }

    let mut all_non_mine_cells_revealed = true;
    for cell in &cell_query {
        if cell.is_mine() { continue }
        if !cell.is_revealed {
            all_non_mine_cells_revealed = false;
            break;
        }
    }

    if all_non_mine_cells_revealed {
        *game_state = MSGameState::GameWin;
    }
}

fn reveal_cell(
    trigger: Trigger<RevealCell>,    
    mut game_state: ResMut<MSGameState>,
    game_board: Res<MSGameBoard>,
    mut cell_query: Query<(&mut MSCell, &mut BackgroundColor)>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
) {
    let triggered_entity = trigger.event().entity;
    let (mut cell, mut background_color) = cell_query.get_mut(triggered_entity).unwrap();

    if cell.is_revealed { return }
    cell.is_revealed = true;

    let grid_size = game_board.grid_size;

    match cell.cell_state {
        MSCellState::Mine => {
            background_color.0 = Color::srgb(1.0, 0.0, 0.0);
            *game_state = MSGameState::GameOver;
        },
        MSCellState::Adjacent(count) => {
            background_color.0 = Color::WHITE;
            if count == 0 {
                let UVec2 { x, y } = cell.pos;
                for ry in y.saturating_sub(1)..=usize::min(y + 1, grid_size - 1) {
                    for rx in x.saturating_sub(1)..=usize::min(x + 1, grid_size - 1) {
                        if ry == y && rx == x { continue }
                        let neighbor_entity = game_board.cells[ry][rx];
                        commands.trigger_targets(RevealCell { entity: neighbor_entity }, neighbor_entity);
                    }
                }
            } else {
                for descendant_entity in children_query.iter_descendants(triggered_entity) {
                    if let Ok(mut text) = text_query.get_mut(descendant_entity) {
                        text.sections[0].value = count.to_string();
                    }
                }
            }
        },
    }

    commands.trigger(CheckWinCondition);
}