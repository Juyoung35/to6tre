use serde::Deserialize;
use std::collections::HashMap;
use ron;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
struct NounBuilder {
    spatial_elements: Vec<SpatialElementBuilder>,
    // shape_builder,
    // pattern_builder,
    // game_specific_element_builder,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
struct VerbBuilder {
    // action_builder,
    // logical_operation_builder,
    // spatial_relation_builder,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub struct GameBuilder {
    #[serde(flatten)]
    nouns: NounBuilder,
    #[serde(flatten)]
    verbs: VerbBuilder,
}



#[derive(Deserialize, Debug)]
struct TextStyleBuilder {
    font: String,
    font_size: f32,
    color: String,
}

#[derive(Deserialize, Debug)]
struct TextSectionBuilder {
    value: String,
    #[serde(flatten)]
    style: TextStyleBuilder,
}

#[derive(Deserialize, Debug)]
struct TextBuilder {
    sections: Vec<TextSectionBuilder>,
    justify: String,
    linebreak_behavior: String,
}

#[derive(Deserialize, Debug)]
struct ElementStyleBuilder {
    #[serde(flatten)]
    text: TextBuilder,
    color: String,
    bg_color: String,
    sprite: String,
    w_per: f64,
    h_per: f64,
}

#[derive(Deserialize, Debug)]
enum Random {
    Default,
    Prob(f64),
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum SpatialElementBuilder {
    Cell(HashMap<String, CellBuilder>),
    Row,
    Column,
    Block,
    Cluster,
    Island,
    Chain,
    Loop,
    Path,
    Corner,
    Edge,
    Border,
    Center,
}

#[derive(Deserialize, Debug)]
struct CellBuilder {
    random: Option<Random>,
    l_click: Option<String>,
    r_click: Option<String>,
    #[serde(flatten)]
    style: ElementStyleBuilder,
    // is_valid: Option<Logic>,
}