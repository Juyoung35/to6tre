use serde::Deserialize;
use ron;

#[derive(Deserialize)]
struct TextStyleBuilder {
    font: String,
    font_size: f32,
    color: String,
}

#[derive(Deserialize)]
struct TextSectionBuilder {
    value: String,
    #[serde(flatten)]
    style: TextStyleBuilder,
}

#[derive(Deserialize)]
struct TextBuilder {
    sections: Vec<TextSectionBuilder>,
    justify: String,
    linebreak_behavior: String,
}

#[derive(Deserialize)]
struct ElementStyleBuilder {
    #[serde(flatten)]
    text: TextBuilder,
    color: String,
    bg_color: String,
    sprite: String,
    w_per: f64,
    h_per: f64,
}

#[derive(Deserialize)]
enum Random {
    Default,
    Prob(f64),
}

#[derive(Deserialize)]
enum SpatialElementBuilder {
    CellBuilder {
        name: String,
        random: Option<Random>,
        l_click: Option<String>,
        r_click: Option<String>,
        #[serde(flatten)]
        style: ElementStyleBuilder,
        // is_valid: Option<Logic>,
    },
    RowBuilder,
    ColumnBuilder,
    BlockBuilder,
    ClusterBuilder,
    IslandBuilder,
    ChainBuilder,
    LoopBuilder,
    PathBuilder,
    CornerBuilder,
    EdgeBuilder,
    BorderBuilder,
    CenterBuilder,
}