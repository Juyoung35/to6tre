use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use ron::{self, ser::PrettyConfig, extensions::Extensions, options::Options};
use bevy::text::BreakLineOn;
use serde_inline_default::serde_inline_default;
use std::cmp::Ordering;
use std::collections::HashMap;
use bevy_mod_picking::prelude::*;
use rand::prelude::*;
use bevy_game::utils::*;

type NounID = usize;

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
    model: NounID,
}

#[derive(Debug)]
enum Noun {
    SpatialElement(SpatialElement),
}

#[derive(Debug)]
enum SpatialElement {
    Cell {
        id: NounID,
        name: String,
        l_click: Option<Action>,
        r_click: Option<Action>,
        style: Style,
        background_color: BackgroundColor,
        border_color: BorderColor,
        border_radius: BorderRadius,
        text: Option<Text>,
    },
}

#[derive(Resource, Debug)]
struct GameBoard {
    config: GameConfig,
    grid: Grid,
}

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<(usize, Entity)>>,
}

#[derive(Debug)]
struct GameConfig {
    name: String,
    nouns: Nouns,
}

#[derive(Debug)]
struct Nouns {
    nouns: Vec<Noun>,
    noun_map: HashMap<String, NounID>,
    gen_config: GenConfig,
}
impl Nouns {
    fn setup(&self, ) {

    }
}



#[derive(Clone, Debug)]
enum GenMethod {
    None,
    Default,
    Random(f64),
    // DependOn,
}

#[derive(Default, Debug)]
struct GenConfig {
    default: NounID,
    probs: Vec<(f64, NounID)>,
}

#[derive(Clone, Copy, Debug)]
enum Action {
    TransformTo(NounID),
}

fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_board: ResMut<GameBoard>,
) {
    // let mut game_board = game_board.into_inner();
    let width = game_board.grid.width;
    let height = game_board.grid.height;
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
            background_color: BackgroundColor(Color::Srgba(WHITE)),
            ..default()
        })
        .with_children(|builder| {
            spawn_header(&mut builder, &font, &game_board.config.name);
            
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
                        grid_template_columns: RepeatedGridTrack::flex(width as u16, 1.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        grid_template_rows: RepeatedGridTrack::flex(height as u16, 1.0),
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

                    let mut rng = rand::thread_rng();
                    let cells = &mut game_board.grid.cells;
                    let probs = &game_board.config.nouns.gen_config.probs;
                    let nouns = &game_board.config.nouns.nouns;
                    for y in 0..height {
                        let mut row = Vec::new();
                        for x in 0..width {
                            let r = rng.gen();
                            let mut gen_id = game_board.config.nouns.gen_config.default;
                            for (prob, noun_id) in probs.iter() {
                                if r > prob { continue }
                                gen_id = *noun_id;
                                break;
                            }
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
                                    Cell { x, y, model: gen_id },
                                ))
                                // .with_children(|builder| {
                                //     builder.spawn(TextBundle::from_section(
                                //         " ",
                                //         TextStyle {
                                //             font: font.clone(),
                                //             font_size: 48.0,
                                //             color: BLACK,
                                //         },
                                //     ));
                                // })
                                .id();
                            row.push((noun_id, entity_id));
                        }
                        cells.push(row);
                    }
                });

            for y in 0..height {
                for x in 0..width {
                    let (noun_id, entity) = game_board.grid.cells[y][x];
                    if let Some(cell) = game_board.config.nouns.nouns.get(noun_id) {

                    }
                    if let Some(mut entity_commands) = commands.get_entity(entity) {
                        entity_commands.insert(
                            On::<Pointer<Click>>::commands_mut(move |click, commands| {
                                match click.button {
                                    PointerButton::Primary => {
                                        commands.trigger_targets(CellClickLeft { x, y }, entity);
                                    },
                                    PointerButton::Secondary => {
                                        commands.trigger_targets(CellClickRight { x, y }, entity);
                                    },
                                    _ => (),
                                }
                            })
                        );
                    }
                }
            }

            spawn_right_side_bar(&mut builder, &font);
            spawn_footer(&mut builder);
        });
}

#[derive(Event)]
struct CellClickLeft {
    x: usize,
    y: usize,
}
impl CellClickLeft {
    fn reveal_cell(
        trigger: Trigger<Self>,
        game_board: ResMut<GameBoard>,
    ) {
        let Self { x, y } = trigger.event();
    }
}

#[derive(Event)]
struct CellClickRight {
    x: usize,
    y: usize,
}

fn detect_change(query: Query<(Entity, &mut NodeBundle, Cell), Changed<Cell>>) {

}

fn merge_node_bundle() {

}

pub fn rr(asset_server: Res<AssetServer>) {
    let r = CellBuilder {
        name: "Tree".to_string(),
        node: NodeBundleBuilder {
            style: Style {
                ..default()
            },
            background_color: "ORANGE".to_string(),
            border_radius: BorderRadius {
                top_left: Val::Px(3.0),
                ..default()
            },
            ..default()
        },
        gen_method: GenMethodBuilder::Random(0.3),
        text: None,
        ..default()
    };
    // let r3 = GameConfigBuilder {
    //     name: "tents and trees".to_string(),
    //     nouns: NounBuilder {
    //         spatial_elements: SpatialElementBuilder {
    //             cells: vec![r],
    //         }
    //     },
    // };
    let options = Options::default()
        .without_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
        .with_default_extension(Extensions::IMPLICIT_SOME);

    // let ss = options.to_string_pretty(&r3, PrettyConfig::default()).unwrap();
    // println!("{ss}");

    let ronfig = r#"
[
    (
        name: "tents and trees",
        nouns: (
            spatial_elements: (
                cells: [
                    {
                        "name": "Empty",
                        "gen_method": "default",
                        "background_color": "WHITE",
                    },
                    {
                        "name": "Flagged",
                        "background_color": "GREEN",
                    },
                    {
                        "name": "Tree",
                        "gen_method": Random(0.3),
                        "background_color": "GREEN",
                    },
                    {
                        "name": "Tent",
                        "background_color": "GREEN",
                    },
                ]
            )
        )
    )
]
"#;

    let mut game_configs = Vec::new();
    let game_config_builders: Vec<GameConfigBuilder> = options.from_str(ronfig).unwrap();
    for game_config_builder in game_config_builders {
        let game_config = game_config_builder.to_game_config(asset_server.clone());
        game_configs.push(game_config);
    }
    let out: Vec<GameConfigBuilder> = options.from_str(ronfig).unwrap();
    println!("{out:?}");
}


// https://github.com/ron-rs/ron/issues/115

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    // InGame(String),
}