use serde::Deserialize;
use ron;

#![enable(implicit_some)]
#[derive(Deserialize)]
enum SpatialElementBuilder {
    CellBuilder {
        name: String,
        rand_args_index: Option<usize>,
        l_click: Option<String>,
        r_click: Option<String>,
        style: ElementStyle,
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