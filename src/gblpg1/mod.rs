// mod utils;
// pub mod parser;
pub mod term;
pub mod term_builder;

use bevy::prelude::*;
use rand::{Rng, rngs::ThreadRng};

use term::Noun;
use term_builder::parse_games;
use utils::*;

type NounID = usize;
const RON_FILE_PATH: &str = "src/gblpg/games.ron";
const WIDTH: usize = 10;
const HEIGHT: usize = 10;

struct GBLPGPlugin;
impl Plugin for GBLPGPlugin {
    fn build(&self, app: &mut App) {
        // app.
        //     ;
    }
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<(NounID, Entity)>>,
    rand_args: Vec<(NounID, f64)>,
}

#[derive(Resource)]
struct Game {
    name: &str,
    nouns: Vec<Noun>,
    grid: Grid,
    rng: ThreadRng,
}
impl Game {
    // fn new() -> Self {
    //     let nouns = Vec::new();
    //     let grid = Grid::new();
    //     let rng = thread_rng();
    //     Self {
    //         nouns,
    //         grid,
    //         rng,
    //     }
    // }
    fn choice_noun(&mut self) -> NounID {
        let x = self.rng.gen();
        for (id, y) in self.grid.rand_args.iter() {
            if x <= y { return id }
        }
        // if failed to return id before, return last index of grid.rand_args
        self.grid.rand_args.len() - 1
    }
}

fn build_game() {
    let games_ron = parse_games(RON_FILE_PATH);
    let mut games = Vec::new();
    for (game_name, game) in games_ron.iter() {
        let GameBuilder { nouns: nouns_ron, verbs_ron } = game;
        let mut rng = thread_rng();
        let mut nouns = Vec::new();
        let mut noun_id = 0;
        let mut rand_args = vec![(0, 1.0)];
        for spatial_element in nouns_ron.spatial_elements {
            match spatial_element {
                SpatialElementBuilder::Cell(map) => {
                    for (name, cell_builder) in map.iter() {
                        let CellBuilder {
                            random_ron,
                            l_click_ron,
                            r_click_ron,
                            style_ron,
                            // is_valid: Option<Logic>,
                        } = cell_builder;
                        let rand_args_index = if let Some(random) = random_ron {
                            Some(match random {
                                Random::Default => {
                                    rand_args[0] = (noun_id, 1.0);
                                    0
                                },
                                Random::Prob(prob) => {
                                    rand_args.push(noun_id, prob);
                                    rand_args.len() - 1
                                },
                            })
                        } else { None };
                        let ElementStyleBuilder {
                            text: TextBuilder,
                            color,
                            bg_color,
                            sprite,
                            w_per,
                            h_per,
                        } = style;
                        nouns.push(
                            Noun::SpatialElement(
                                SpatialElement::Cell {
                                    name: name.as_str(),
                                    rand_args_index,
                                    l_click: None,
                                    r_click: None,
                                    style,
                                }
                            )
                        );
                    }
                },
                _ => (),
            }
            noun_id += 1;
        }
        games.push(Game {
            name: game_name,

        })
    }
}

fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_board: ResMut<GameBoard>,
) {
    let mut grid = &mut game_board.grid;
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
            background_color: BackgroundColor(WHITE),
            ..default()
        })
        .with_children(|builder| {
            spawn_header(&mut builder, &font);
            
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
                        grid_template_columns: RepeatedGridTrack::flex(grid.size as u16, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(grid.size as u16, 1.0),
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

                    let cells = &mut grid.cells;
                    for y in 0..grid.size {
                        cells.push(Vec::new());
                        for x in 0..grid.size {
                            let cell_type = ();
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
                                            color: BLACK,
                                        },
                                    ));
                                })
                                .id();
                            cells[y].push(entity_id);
                        }
                    }
                });
            
            spawn_right_side_bar(&mut builder, &font);
            spawn_footer(&mut builder);
        });
}