// GPG stands for Grid-based Puzzle Gameuse bevy::prelude::*;

use bevy::prelude::*;
use bevy::color::palettes::css::*;
use bevy_mod_picking::prelude::*;

use web_sys::console;
use rand::Rng;

use std::fmt::Debug;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point<T>
where T: Clone + Copy + Debug + PartialEq
{
    x: T,
    y: T,
}
impl<T> Point<T>
where T: Clone + Copy + Debug + PartialEq
{
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, Default, Component)]
struct Cell<S: CellState> {
    pos: Point<usize>,
    cell_state: S,
}
impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Self {
            pos: Point::new(x, y),
            ..default()
        }
    }
}

// https://users.rust-lang.org/t/how-to-parse-enum-macro/36161/4
// #![feature(trace_macros)]
// trace_macros! {true}

macro_rules! build_cell_state {
    // VariantName
    (
        @name $name:ident
        @variants [
            $($variants:tt)*
        ]
        @parsing
            $VariantName:ident
            $(, $($input:tt)*)?
    ) => (build_cell_state! {
        @name $name
        @variants [
            $($variants)*
            {
                $VariantName
            }
        ]
        @parsing
            $( $($input)* )?
    });

    // VariantName(...)
    (
        @name $name:ident
        @variants [
            $($variants:tt)*
        ]
        @parsing
            $VariantName:ident ( $($tt:tt)* )
            $(, $($input:tt)*)?
    ) => (build_cell_state! {
        @name $name
        @variants [
            $($variants)*
            {
                $VariantName ($($tt)*)
            }
        ]
        @parsing
            $( $($input)* )?
    });

    // VariantName { ... }
    (
        @name $name:ident
        @variants [
            $($variants:tt)*
        ]
        @parsing
            $VariantName:ident { $($tt:tt)* }
            $(, $($input:tt)*)?
    ) => (build_cell_state! {
        @name $name
        @variants [
            $($variants)*
            {
                $VariantName { $($tt)* }
            }
        ]
        @parsing
            $( $($input)* )?
    });

    // Done parsing, time to generate code:
    (
        @name $name:ident
        @variants [
            $(
                {
                    $VariantName:ident $($variant_assoc:tt)?
                }
            )*
        ]
        @parsing
            // Nothing left to parse
    ) => (
        #[derive(Clone, Debug)]
        enum $name {
            $(
                $VariantName $(
                    $variant_assoc
                )? ,
            )*
        }
        impl CellState for $name {
            
        }
    );

    // == ENTRY POINT ==
    (
        enum $name:ident {
            $($input:tt)*
        }
    ) => (build_cell_state! {
        @name $name
        // a sequence of brace-enclosed variants
        @variants []
        // remaining tokens to parse
        @parsing
            $($input)*
    });
}

// variant -> {left_next, right_next} => {render}



trait CellState {

}
build_cell_state!(
    enum MSCellState {
        Mine,
        // #[default]
        Adjacent(usize),
    }
);

#[derive(Event)]
struct CellLeftClick {
    pos: Point<usize>,
}

#[derive(Event)]
struct CellRightClick {
    pos: Point<usize>,
}

#[derive(Event)]
struct CheckWinCondition;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    // MainMenu,
    // SettingsMenu,
    InGame,
    GameWin,
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
                        grid_template_columns: RepeatedGridTrack::flex(grid_size as u16, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(grid_size as u16, 1.0),
                        // Set a 12px gap/gutter between rows and columns
                        row_gap: Val::Px(1.0),
                        column_gap: Val::Px(1.0),
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
                                    PickableBundle::default(),
                                    NodeBundle {
                                        style: Style {
                                            display: Display::Grid,
                                            padding: UiRect::all(Val::Px(3.0)),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(Color::srgb(0.75, 0.75, 0.75)),
                                        ..default()
                                    },
                                    MSCell::new(x, y),
                                ))
                                .with_children(|builder| {
                                    builder.spawn(TextBundle::from_section(
                                        " ",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 48.0,
                                            color: Color::BLACK,
                                        },
                                    ));
                                })
                                // .with_children(|builder| {

                                // })
                                .id();
                            cells[y].push(entity_id);
                        }
                    }
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
                        "Game State",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                    ));
                    builder.spawn(TextBundle::from_section(
                        "Revealed : 0 / 0",
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
                                commands.trigger_targets(CellLeftClick { pos: Point::new(x, y) }, entity);
                            },
                            PointerButton::Secondary => {
                                commands.trigger_targets(CellRightClick { pos: Point::new(x, y) }, entity);
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