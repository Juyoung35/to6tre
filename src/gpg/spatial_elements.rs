#![enable(implicit_some)]
#[serde(default)]
#[derive(Serialize, Deserialize, Debug)]
struct CellBuilder {
    is_default: Option<bool>,
    l_click: Option<String>,
    r_click: Option<String>,
    color: Option<String>,
    sprite: Option<String>,
    is_valid: Option<String>,
}
// expand click to handle action
struct Cell {
    name: String,
    is_default: bool,
    l_click: Option<ClickAction>,
    r_click: Option<ClickAction>,
    color: Color,
    sprite: String,
    is_valid: Option<Logic>,
}
impl Cell {
    fn from_builder(name: String, builder: CellBuilder, cell_terms: &Vec<CellBuilder>, cell_map: &HashMap<String, usize>) -> Self {
        let is_default = builder.is_default.unwrap_or(false);
        if is_default {

        }
        let color = str_to_css_color(if let Some(color_str) = builder.color {
            color_str.to_uppercase().to_str()
        } else { "WHITE" });
        Self {
            name,
            is_default,
            l_click,
            r_click,
            color,
            sprite,
            is_valid,
        }
    }
}