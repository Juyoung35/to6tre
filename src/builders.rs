use bevy::prelude::*;
use bevy::text::BreakLineOn;
use serde::{Serialize, Deserialize};
use serde_inline_default::serde_inline_default;
use std::collections::HashMap;
use std::cmp::Ordering;
use crate::g::*;
use crate::utils::*;

#[derive(Serialize, Deserialize, Default, Debug)]
pub(super) struct GameConfigBuilder {
    name: String,
    noun_builder: NounBuilder,
}
impl GameConfigBuilder {
    pub(super) fn to_game_config(self, asset_server: AssetServer) -> GameConfig {
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub(super) enum GenMethodBuilder {
    #[default]
    None,
    Default,
    Random(f64),
    // DependOn(String),
}

#[derive(Serialize, Deserialize, Debug)]
enum ActionBuilder {
    TransformTo(String),
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
struct CellBuilder {
    name: String,
    l_click: Option<ActionBuilder>,
    r_click: Option<ActionBuilder>,
    // valid: 
    gen_method: GenMethodBuilder,
    style: Style,
    #[serde_inline_default("WHITE".to_string())]
    background_color: String,
    #[serde_inline_default("BLACK".to_string())]
    border_color: String,
    border_radius: BorderRadius,
    text: Option<TextBuilder>,
}
// impl Builder for CellBuilder {
impl CellBuilder {
    fn to_noun(self, asset_server: AssetServer, noun_map: &HashMap<String, usize>) -> Noun {
        let Self { name, l_click, r_click, style, background_color, border_color, border_radius, text: text_ops, .. } = self;
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
                style,
                background_color: BackgroundColor(Color::Srgba(str_to_css_srgba(&background_color))),
                border_color: BorderColor(Color::Srgba(str_to_css_srgba(&border_color))),
                border_radius,
                text: if let Some(text) = text_ops { Some(text.to_text(asset_server)) } else { None },
            }
        )
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
            color: Color::Srgba(str_to_css_srgba(&color)),
        }
    }
}