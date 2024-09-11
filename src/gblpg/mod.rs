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