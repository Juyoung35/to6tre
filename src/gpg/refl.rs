use serde::{Serialize, Deserialize};

struct GameElementsBuilder {
    spatial_elements_builder: SpatialElementsBuilder,
}
struct SpatialElementsBuilder {
    cells: HashMap<String, CellBuilder>,
}
// id, resource, hashmap -> id

struct ElementID(usize);
// struct ElementIDVisitor;
// impl<'de> Visitor<'de> for I32Visitor {
//     type Value = ElementID;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         formatter.write_str("an integer between -2^31 and 2^31")
//     }
// }
// impl<'de> Deserialize<'de> for ElementID {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_i32(I32Visitor)
//     }
// }


enum Logic {
    And,
    Or,
    Not,
    Verb(),
}

fn parse_spatial_elements() -> std::io::Result<()> {
    let mut contents = String::new();
    read_ron(&mut contents);
    let games: HashMap<String, GameElements> = ron::from_str(contents).unwrap();
    for (game_name, game_elements) in games {
        
    }
    Ok(())
}

fn read_ron(contents: &mut String) -> std::io::Result<()> {
    let file = File::open("games.ron")?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(contents)?;
    Ok(())
}

struct Color;
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum SpatialElement {
    // Grid
    Cell {
        name: String,
        default: bool,
        l_click: String,
        r_click: String,
        color: String,
        sprite: String,
        is_valid: String,
    },
    OutterBoard {
        template: String,
    },
    InCell {
        diag: Diag, // 방향 조합
    },
    Pair,
    Row,
    Column,
    Block, // rectangle
    Area,
    ContiniousIsland, // cluster, region horz / vert / diag
    Chain,
    Loop,
    Path,
    Corner,
    Edge,
    // Border -> just is_border, get_border (perimeter of grid)
    // Center -> just is_center, get_center
}

enum ClickAction {
    TransformTo(ElementID),
}