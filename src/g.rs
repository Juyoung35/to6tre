use bevy::prelude::*;
use bevy::color::palettes::css::*;
use serde::{Serialize, Deserialize};
use ron::{self, ser::PrettyConfig, extensions::Extensions, options::Options};
use bevy::text::BreakLineOn;
use serde_inline_default::serde_inline_default;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
    model: usize,
}

// struct CellModel {
//     id: usize,
//     name: String,
//     class: Noun,
//     // actions,
//     node_bundle: NodeBundle,
//     text: Option<Text>,
// }

enum Noun {
    SpatialElement(SpatialElement),
}
enum SpatialElement {
    Cell {
        id: usize,
        name: String,
        l_click: Option<Action>,
        r_click: Option<Action>,
        node_bundle: NodeBundle,
        text: Option<Text>,
    },
}
// trait Builder {
//     fn to_noun(self, asset_server: Res<AssetServer>) -> Noun;
// }

struct GameBoard {
    config: GameConfig,
    grid: Grid,
}

struct Grid {
    height: usize,
    width: usize,
    cells: Vec<Vec<Entity>>,
}

struct GameConfig {
    name: String,
    nouns: Nouns,
}

struct Nouns {
    nouns: Vec<Noun>,
    noun_map: HashMap<String, usize>,
    gen_config: GenConfig,
}
impl Nouns {
    fn setup(&self, ) {

    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct GameConfigBuilder {
    name: String,
    noun_builder: NounBuilder,
}
impl GameConfigBuilder {
    fn to_game_config(self, asset_server: AssetServer) -> GameConfig {
        let Self { name, noun_builder } = self;
        GameConfig {
            name,
            nouns: noun_builder.to_nouns(asset_server),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct NounBuilder {
    spatial_element_builder: SpatialElementBuilder,
}
impl NounBuilder {
    fn to_nouns(self, asset_server: AssetServer) -> Nouns {
        let mut nouns = Vec::new();
        let Self { spatial_element_builder } = self;
        let SpatialElementBuilder { cells } = spatial_element_builder;
        let mut noun_id = 0;
        let mut noun_map = HashMap::new();
        let mut gen_config = GenConfig::default();
        for cell in cells.iter() {
            match cell.gen_method {
                GenMethodBuilder::None => (),
                GenMethodBuilder::Default => gen_config.default = noun_id,
                GenMethodBuilder::Random(mut prob) => {
                    if let Some((last_prob, _)) = gen_config.probs.last() {
                        prob += last_prob;
                    }
                    gen_config.probs.push((prob, noun_id));
                },
            }
            noun_map.insert(cell.name.clone(), noun_id);
            noun_id += 1;
        }
        let (last_prob, is_greater) = if let Some((last_prob, _)) = gen_config.probs.last() {
            
            match f64::partial_cmp(last_prob, &1.0) {
                Some(Ordering::Greater) => (*last_prob, true),
                _ => (*last_prob, false),
            }
        } else { (0.0, false) };
        if is_greater {
            for (prob, _) in gen_config.probs.iter_mut() {
                *prob /= last_prob;
            }
        }
        for cell in cells {
            nouns.push(cell.to_noun(asset_server.clone(), &noun_map));
        }
        Nouns {
            nouns,
            noun_map,
            gen_config,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SpatialElementBuilder {
    cells: Vec<CellBuilder>,
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
    default: usize,
    probs: Vec<(f64, usize)>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
enum GenMethodBuilder {
    #[default]
    None,
    Default,
    Random(f64),
    // DependOn(String),
}
// impl GenMethodBuilder {
//     fn to_gen_method(self, noun_map: &HashMap<String, usize>, rng_vec: &mut Vec<usize>) -> GenMethod {

//     }
// }

#[derive(Clone, Copy, Debug)]
enum Action {
    TransformTo(usize),
}

#[derive(Serialize, Deserialize, Debug)]
enum ActionBuilder {
    TransformTo(String),
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
struct CellBuilder {
    name: String,
    l_click: Option<ActionBuilder>,
    r_click: Option<ActionBuilder>,
    // valid: 
    gen_method: GenMethodBuilder,
    #[serde(flatten)]
    node: NodeBundleBuilder,
    text: Option<TextBuilder>,
}
// impl Builder for CellBuilder {
impl CellBuilder {
    fn to_noun(self, asset_server: AssetServer, noun_map: &HashMap<String, usize>) -> Noun {
        let Self { name, l_click, r_click, node, text: text_ops, .. } = self;
        Noun::SpatialElement(
            SpatialElement::Cell {
                id: *noun_map.get(&name).unwrap(),
                name,
                l_click: match l_click {
                    Some(ActionBuilder::TransformTo(next)) => {
                        Some(Action::TransformTo(*noun_map.get(&next).unwrap()))
                    },
                    _ => None,
                },
                r_click: match r_click {
                    Some(ActionBuilder::TransformTo(next)) => {
                        Some(Action::TransformTo(*noun_map.get(&next).unwrap()))
                    },
                    _ => None,
                },
                node_bundle: node.to_node_bundle(),
                text: if let Some(text) = text_ops { Some(text.to_text(asset_server)) } else { None },
            }
        )
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Default, Debug)]
struct NodeBundleBuilder {
    #[serde(default)]
    style: Style,
    #[serde_inline_default("WHITE".to_string())]
    background_color: String,
    #[serde_inline_default("BLACK".to_string())]
    border_color: String,
    #[serde(default)]
    border_radius: BorderRadius,
}
impl NodeBundleBuilder {
    fn to_node_bundle(self) -> NodeBundle {
        let Self { style, background_color, border_color, border_radius } = self;
        NodeBundle {
            style,
            background_color: BackgroundColor(Color::Srgba(str_to_css_color(&background_color))),
            border_color: BorderColor(Color::Srgba(str_to_css_color(&border_color))),
            border_radius,
            ..default()
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
struct TextBuilder {
    section_builders: Vec<TextSectionBuilder>,
    justify: JustifyText,
    linebreak_behavior: BreakLineOn,
}
impl TextBuilder {
    fn to_text(self, asset_server: AssetServer) -> Text {
        let Self { section_builders, justify, linebreak_behavior } = self;
        let mut sections = Vec::new();
        for section_builder in section_builders {
            sections.push(section_builder.to_text_section(asset_server.clone()));
        }
        Text {
            sections,
            justify,
            linebreak_behavior,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
struct TextSectionBuilder {
    value: String,
    style_builder: TextStyleBuilder,
}
impl TextSectionBuilder {
    fn to_text_section(self, asset_server: AssetServer) -> TextSection {
        let Self { value, style_builder } = self;
        TextSection {
            value,
            style: style_builder.to_text_style(asset_server),
        }
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Default, Debug)]
struct TextStyleBuilder {
    #[serde_inline_default("fonts/FiraSans-Bold.ttf".to_string())]
    font: String,
    #[serde_inline_default(24.0)]
    font_size: f32,
    #[serde_inline_default("BLACK".to_string())]
    color: String,
}
impl TextStyleBuilder {
    fn to_text_style(self, asset_server: AssetServer) -> TextStyle {
        let Self { font, font_size, color } = self;
        TextStyle {
            font: asset_server.load(font),
            font_size,
            color: Color::Srgba(str_to_css_color(&color)),
        }
    }
}

fn str_to_css_color(color_str: &str) -> Srgba {
    match color_str {
        "ALICE_BLUE" => ALICE_BLUE,
        "ANTIQUE_WHITE" => ANTIQUE_WHITE,
        "AQUA" => AQUA,
        "AQUAMARINE" => AQUAMARINE,
        "AZURE" => AZURE,
        "BEIGE" => BEIGE,
        "BISQUE" => BISQUE,
        "BLACK" => BLACK,
        "BLANCHED_ALMOND" => BLANCHED_ALMOND,
        "BLUE" => BLUE,
        "BLUE_VIOLET" => BLUE_VIOLET,
        "BROWN" => BROWN,
        "BURLYWOOD" => BURLYWOOD,
        "CADET_BLUE" => CADET_BLUE,
        "CHARTREUSE" => CHARTREUSE,
        "CHOCOLATE" => CHOCOLATE,
        "CORAL" => CORAL,
        "CORNFLOWER_BLUE" => CORNFLOWER_BLUE,
        "CORNSILK" => CORNSILK,
        "CRIMSON" => CRIMSON,
        "DARK_BLUE" => DARK_BLUE,
        "DARK_CYAN" => DARK_CYAN,
        "DARK_GOLDENROD" => DARK_GOLDENROD,
        "DARK_GRAY" => DARK_GRAY,
        "DARK_GREEN" => DARK_GREEN,
        "DARK_GREY" => DARK_GREY,
        "DARK_KHAKI" => DARK_KHAKI,
        "DARK_MAGENTA" => DARK_MAGENTA,
        "DARK_OLIVEGREEN" => DARK_OLIVEGREEN,
        "DARK_ORANGE" => DARK_ORANGE,
        "DARK_ORCHID" => DARK_ORCHID,
        "DARK_RED" => DARK_RED,
        "DARK_SALMON" => DARK_SALMON,
        "DARK_SEA_GREEN" => DARK_SEA_GREEN,
        "DARK_SLATE_BLUE" => DARK_SLATE_BLUE,
        "DARK_SLATE_GRAY" => DARK_SLATE_GRAY,
        "DARK_SLATE_GREY" => DARK_SLATE_GREY,
        "DARK_TURQUOISE" => DARK_TURQUOISE,
        "DARK_VIOLET" => DARK_VIOLET,
        "DEEP_PINK" => DEEP_PINK,
        "DEEP_SKY_BLUE" => DEEP_SKY_BLUE,
        "DIM_GRAY" => DIM_GRAY,
        "DIM_GREY" => DIM_GREY,
        "DODGER_BLUE" => DODGER_BLUE,
        "FIRE_BRICK" => FIRE_BRICK,
        "FLORAL_WHITE" => FLORAL_WHITE,
        "FOREST_GREEN" => FOREST_GREEN,
        "FUCHSIA" => FUCHSIA,
        "GAINSBORO" => GAINSBORO,
        "GHOST_WHITE" => GHOST_WHITE,
        "GOLD" => GOLD,
        "GOLDENROD" => GOLDENROD,
        "GRAY" => GRAY,
        "GREEN" => GREEN,
        "GREEN_YELLOW" => GREEN_YELLOW,
        "GREY" => GREY,
        "HONEYDEW" => HONEYDEW,
        "HOT_PINK" => HOT_PINK,
        "INDIAN_RED" => INDIAN_RED,
        "INDIGO" => INDIGO,
        "IVORY" => IVORY,
        "KHAKI" => KHAKI,
        "LAVENDER" => LAVENDER,
        "LAVENDER_BLUSH" => LAVENDER_BLUSH,
        "LAWN_GREEN" => LAWN_GREEN,
        "LEMON_CHIFFON" => LEMON_CHIFFON,
        "LIGHT_BLUE" => LIGHT_BLUE,
        "LIGHT_CORAL" => LIGHT_CORAL,
        "LIGHT_CYAN" => LIGHT_CYAN,
        "LIGHT_GOLDENROD_YELLOW" => LIGHT_GOLDENROD_YELLOW,
        "LIGHT_GRAY" => LIGHT_GRAY,
        "LIGHT_GREEN" => LIGHT_GREEN,
        "LIGHT_GREY" => LIGHT_GREY,
        "LIGHT_PINK" => LIGHT_PINK,
        "LIGHT_SALMON" => LIGHT_SALMON,
        "LIGHT_SEA_GREEN" => LIGHT_SEA_GREEN,
        "LIGHT_SKY_BLUE" => LIGHT_SKY_BLUE,
        "LIGHT_SLATE_GRAY" => LIGHT_SLATE_GRAY,
        "LIGHT_SLATE_GREY" => LIGHT_SLATE_GREY,
        "LIGHT_STEEL_BLUE" => LIGHT_STEEL_BLUE,
        "LIGHT_YELLOW" => LIGHT_YELLOW,
        "LIME" => LIME,
        "LIMEGREEN" => LIMEGREEN,
        "LINEN" => LINEN,
        "MAGENTA" => MAGENTA,
        "MAROON" => MAROON,
        "MEDIUM_AQUAMARINE" => MEDIUM_AQUAMARINE,
        "MEDIUM_BLUE" => MEDIUM_BLUE,
        "MEDIUM_ORCHID" => MEDIUM_ORCHID,
        "MEDIUM_PURPLE" => MEDIUM_PURPLE,
        "MEDIUM_SEA_GREEN" => MEDIUM_SEA_GREEN,
        "MEDIUM_SLATE_BLUE" => MEDIUM_SLATE_BLUE,
        "MEDIUM_SPRING_GREEN" => MEDIUM_SPRING_GREEN,
        "MEDIUM_TURQUOISE" => MEDIUM_TURQUOISE,
        "MEDIUM_VIOLET_RED" => MEDIUM_VIOLET_RED,
        "MIDNIGHT_BLUE" => MIDNIGHT_BLUE,
        "MINT_CREAM" => MINT_CREAM,
        "MISTY_ROSE" => MISTY_ROSE,
        "MOCCASIN" => MOCCASIN,
        "NAVAJO_WHITE" => NAVAJO_WHITE,
        "NAVY" => NAVY,
        "OLD_LACE" => OLD_LACE,
        "OLIVE" => OLIVE,
        "OLIVE_DRAB" => OLIVE_DRAB,
        "ORANGE" => ORANGE,
        "ORANGE_RED" => ORANGE_RED,
        "ORCHID" => ORCHID,
        "PALE_GOLDENROD" => PALE_GOLDENROD,
        "PALE_GREEN" => PALE_GREEN,
        "PALE_TURQUOISE" => PALE_TURQUOISE,
        "PALE_VIOLETRED" => PALE_VIOLETRED,
        "PAPAYA_WHIP" => PAPAYA_WHIP,
        "PEACHPUFF" => PEACHPUFF,
        "PERU" => PERU,
        "PINK" => PINK,
        "PLUM" => PLUM,
        "POWDER_BLUE" => POWDER_BLUE,
        "PURPLE" => PURPLE,
        "REBECCA_PURPLE" => REBECCA_PURPLE,
        "RED" => RED,
        "ROSY_BROWN" => ROSY_BROWN,
        "ROYAL_BLUE" => ROYAL_BLUE,
        "SADDLE_BROWN" => SADDLE_BROWN,
        "SALMON" => SALMON,
        "SANDY_BROWN" => SANDY_BROWN,
        "SEASHELL" => SEASHELL,
        "SEA_GREEN" => SEA_GREEN,
        "SIENNA" => SIENNA,
        "SILVER" => SILVER,
        "SKY_BLUE" => SKY_BLUE,
        "SLATE_BLUE" => SLATE_BLUE,
        "SLATE_GRAY" => SLATE_GRAY,
        "SLATE_GREY" => SLATE_GREY,
        "SNOW" => SNOW,
        "SPRING_GREEN" => SPRING_GREEN,
        "STEEL_BLUE" => STEEL_BLUE,
        "TAN" => TAN,
        "TEAL" => TEAL,
        "THISTLE" => THISTLE,
        "TOMATO" => TOMATO,
        "TURQUOISE" => TURQUOISE,
        "VIOLET" => VIOLET,
        "WHEAT" => WHEAT,
        "WHITE" => WHITE,
        "WHITE_SMOKE" => WHITE_SMOKE,
        "YELLOW" => YELLOW,
        "YELLOW_GREEN" => YELLOW_GREEN,
        _ => WHITE,
    }
}

fn spawn_header(builder: &mut ChildBuilder, font: &Font, game_title: &str) {
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
            spawn_nested_text_bundle(builder, font.clone(), game_title);
        });
}

fn spawn_right_side_bar(builder: &mut ChildBuilder, font: &Font) {
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
            background_color: BackgroundColor(BLACK),
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
}

fn spawn_footer(builder: &mut ChildBuilder) {
    // Footer / status bar
    builder.spawn(NodeBundle {
        style: Style {
            // Make this node span two grid column so that it takes up the entire bottom row
            grid_column: GridPlacement::span(2),
            ..default()
        },
        background_color: BackgroundColor(WHITE),
        ..default()
    });
}

fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_board: ResMut<GameBoard>,
) {
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
            background_color: BackgroundColor(WHITE),
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
                    let cells = &mut grid.cells;
                    let probs = &game_board.game_config.nouns.gen_config.probs;
                    let nouns = &game_board.game_config.nouns.nouns;
                    for y in 0..grid.height {
                        let mut rows = Vec::new();
                        for x in 0..grid.width {
                            let r = rng.gen();
                            let mut gen_id = game_board.game_config.nouns.gen_config.default;
                            for (prob, noun_id) in probs.iter() {
                                if r > prob { continue }
                                gen_id = noun_id;
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
                            rows.push(entity_id);
                        }
                        cells.push(row);
                    }
                });

            for y in 0..height {
                for x in 0..width {

                }
            }

            spawn_right_side_bar(&mut builder, &font);
            spawn_footer(&mut builder);
        });
}

fn detect_change(query: Query<)

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

pub fn parse_games(asset_server: Res<AssetServer>) {
    if Ok(game_config_builders) = read_games("src/games.ron") {
        let mut game_configs = Vec::new();
        for game_config_builder in game_config_builders {
            let game_config = game_config_builder.to_game_config(asset_server.clone());
            game_configs.push(game_config);
        }
    } else {
        panic!("Failed to Parse Game!");
    }
}

fn read_games(file_path: &str) -> std::io::Result<Vec<GameConfigBuilder>> {
    // Setup the options
    let options = Options::default()
        .without_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
        .with_default_extension(Extensions::IMPLICIT_SOME);

    let mut contents = String::new();
    read_ron(&mut contents, file_path);
    let game_config_builders: Vec<GameConfigBuilder> = options.from_str(&mut contents).unwrap();

    Ok(game_config_builders)
}

fn read_ron(contents: &mut String, file_path: &str) -> std::io::Result<()> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(contents)?;
    Ok(())
}