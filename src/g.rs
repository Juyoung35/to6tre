use bevy::prelude::*;
use bevy::color::palettes::css::*;
use serde::{Serialize, Deserialize};
use ron::{self, ser::PrettyConfig, extensions::Extensions, options::Options};
use bevy::text::BreakLineOn;
use serde_inline_default::serde_inline_default;

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
    model: usize,
}

struct CellModel {
    id: usize,
    name: String,
    // actions,
    node_bundle: NodeBundle,
    text: Option<Text>,
}

// trait Builder {
//     type Model;
//     fn to_model() -> Self::Model;
// }

#[derive(Serialize, Deserialize, Default, Debug)]
struct GameConfigBuilder {
    name: String,
    nouns: NounBuilder,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct NounBuilder {
    spatial_elements: SpatialElementBuilder,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SpatialElementBuilder {
    cells: Vec<CellBuilder>,
}
impl SpatialElementBuilder {

}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
struct CellBuilder {
    name: String,
    // l_click: Option<Action>,
    // r_click: Option<Action>,
    #[serde(flatten)]
    node: NodeBundleBuilder,
    text: Option<TextBuilder>,
}
impl CellBuilder {
    fn to_cell_model(self, asset_server: Res<AssetServer>) -> CellModel {
        let Self { name, node, text: text_ops } = self;
        CellModel {
            id: 0,
            name,
            node_bundle: node.to_node_bundle(),
            text: if let Some(text) = text_ops { Some(text.to_text(asset_server)) } else { None },
        }
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
    fn to_text(self, asset_server: Res<AssetServer>) -> Text {
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

pub fn rr() {
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
        text: None,
    };
    let r3 = GameConfigBuilder {
        name: "tents and trees".to_string(),
        nouns: NounBuilder {
            spatial_elements: SpatialElementBuilder {
                cells: vec![r],
            }
        },
    };
    let options = Options::default()
        .without_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
        .with_default_extension(Extensions::IMPLICIT_SOME);

    let ss = options.to_string_pretty(&r3, PrettyConfig::default()).unwrap();
    println!("{ss}");

    let s = r#"
    [
        {
            "name": "Empty",
            "background_color": "WHITE",
        },
        {
            "name": "Flagged",
            "background_color": "GREEN",
        },
        {
            "name": "Tree",
            "background_color": "GREEN",
        },
        {
            "name": "Tent",
            "background_color": "GREEN",
        },
    ]
    "#;

    let out: Vec<CellBuilder> = options.from_str(s).unwrap();
    println!("{out:?}");
}

// https://github.com/ron-rs/ron/issues/115

// r#"
// (
//     name: "tents and trees",
//     nouns: (
//         spatial_elements: (
//             cells: [
//                 {
//                     "name": "Tree",
//                     "style": (
//                     )
//                 }
//             ]
//         )
//     )
// )
// "#;