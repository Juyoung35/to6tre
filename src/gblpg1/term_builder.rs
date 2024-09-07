use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use ron::{self, extensions::Extensions, options::Options};

fn read_ron(contents: &mut String, file_path: &str) -> std::io::Result<()> {
    // let file = File::open("games.ron")?;
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(contents)?;
    Ok(())
}

pub(super) fn parse_games(file_path: &str) -> std::io::Result<HashMap<String, GameBuilder>> {
    // Setup the options
    let options = Options::default()
        .without_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
        .with_default_extension(Extensions::IMPLICIT_SOME);

    let mut contents = String::new();
    read_ron(&mut contents, file_path);
    let games: HashMap<String, GameBuilder> = options.from_str(&mut contents).unwrap();

    Ok(games)
}

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
pub(super) struct GameBuilder {
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
impl Default for TextStyleBuilder {
    fn default() -> Self {
        Self {
            font: "fonts/FiraSans-Bold.ttf",
            font_size: 32.0,
            color: "WHITE",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
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
pub(super) enum SpatialElementBuilder {
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
pub(super) struct CellBuilder {
    random: Option<Random>,
    l_click: Option<String>,
    r_click: Option<String>,
    #[serde(flatten)]
    style: ElementStyleBuilder,
    // is_valid: Option<Logic>,
}

// pub(super) fn test() {
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