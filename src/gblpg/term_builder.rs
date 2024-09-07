use serde::{Serialize, Deserialize};
use std::collections::HashMap;
// use ron;
use ron::{self, extensions::Extensions, options::Options};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct NounBuilder {
    spatial_elements: Vec<SpatialElementBuilder>,
    // shape_builder,
    // pattern_builder,
    // game_specific_element_builder,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct VerbBuilder {
    // action_builder,
    // logical_operation_builder,
    // spatial_relation_builder,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct GameBuilder {
    #[serde(flatten)]
    nouns: NounBuilder,
    #[serde(flatten)]
    verbs: VerbBuilder,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextStyleBuilder {
    font: String,
    font_size: f32,
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextSectionBuilder {
    value: String,
    #[serde(flatten)]
    style: TextStyleBuilder,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct TextBuilder {
    sections: Vec<TextSectionBuilder>,
    justify: String,
    linebreak_behavior: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct ElementStyleBuilder {
    #[serde(flatten)]
    text: TextBuilder,
    color: String,
    bg_color: String,
    sprite: String,
    w_per: f64,
    h_per: f64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
enum Random {
    #[default]
    Default,
    Prob(f64),
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct CellBuilder {
    random: Option<Random>,
    l_click: Option<String>,
    r_click: Option<String>,
    #[serde(flatten)]
    style: ElementStyleBuilder,
    // is_valid: Option<Logic>,
}

// pub fn test() {
//     let options = Options::default()
//         .without_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
//         .with_default_extension(Extensions::IMPLICIT_SOME);
//     let mut map = HashMap::new();
//     map.insert("TreeGame".to_string(), GameBuilder {
//         nouns: NounBuilder {
//             spatial_elements: vec![
//                 SpatialElementBuilder::Cell({
//                     let mut m = HashMap::new();
//                     m.insert("Empty".to_string(), CellBuilder {

//                     });
//                     m
//                 })
//             ],
//         },
//         ..Default::default()
//     });

//     let s = options.to_string(&map).unwrap();
//     println!("{s:?}");
// }