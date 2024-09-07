mod utils;

use serde_inline_default;

#[derive(Resource)]
struct Game {
    name: &str,
    nouns: Vec<Noun>,
    grid: Grid,
    rng: ThreadRng,
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<(NounID, Entity)>>,
    rand_args: Vec<(NounID, f64)>,
}

enum Noun {
    SpatialElement(SpatialElement),
    Shape(Shape),
    Pattern(Pattern),
    GameSpecificElement(GameSpecificElement),
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub(super) struct GameBuilder {
    #[serde(flatten)]
    nouns: NounBuilder,
    #[serde(flatten)]
    verbs: VerbBuilder,
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

#[derive(Serialize, Deserialize, Debug)]
enum Random {
    Default,
    Prob(f64),
}

#[serde_inline_default]
#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
struct ElementStyleBuilder {
    #[serde(flatten)]
    text: Option<TextBuilder>,
    #[serde_inline_default("WHITE")]
    color: String,
    #[serde_inline_default("WHITE")]
    bg_color: String,
    sprite: Option<String>,
    #[serde_inline_default(1.0)]
    w_per: f64,
    #[serde_inline_default(1.0)]
    h_per: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
struct TextBuilder {
    sections: Vec<TextSectionBuilder>,
    #[serde_inline_default("Center")]
    justify: String,
    #[serde_inline_default("WordBoundary")]
    linebreak_behavior: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct TextSectionBuilder {
    value: String,
    #[serde(flatten)]
    style: TextStyleBuilder,
}

#[derive(Serialize, Deserialize, Debug)]
struct TextStyleBuilder {
    #[serde_inline_default("fonts/FiraSans-Bold.ttf")]
    font: String,
    #[serde_inline_default(32.0)]
    font_size: f32,
    #[serde_inline_default("WHITE")]
    color: String,
}