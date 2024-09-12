use bevy::prelude::*;
use bevy::color::palettes::css::*;
use ron::{self, extensions::Extensions, options::Options};
use std::io::{BufReader, Read};
use std::fs::File;
use crate::g::GameConfigs;
use crate::builders::GameConfigBuilder;

pub(super) fn parse_games(
    asset_server: Res<AssetServer>,
    mut game_configs: ResMut<GameConfigs>,
) {
    if let Ok(game_config_builders) = read_games("src/games.ron") {
        for game_config_builder in game_config_builders {
            let game_config = game_config_builder.to_game_config(asset_server.clone());
            game_configs.0.insert(game_config.name.clone(), game_config);
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



pub(super) fn spawn_header(builder: &mut ChildBuilder, font: &Handle<Font>, game_title: &str) {
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

pub(super) fn spawn_right_side_bar(builder: &mut ChildBuilder, font: &Handle<Font>) {
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
}

pub(super) fn spawn_footer(builder: &mut ChildBuilder) {
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
}

pub(super) fn spawn_nested_text_bundle(builder: &mut ChildBuilder, font: Handle<Font>, text: &str) {
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



pub(super) fn str_to_css_srgba(color_str: &str) -> Srgba {
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