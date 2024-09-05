use super::term::*;

enum Noun {
    SpatialElement(SpatialElement),
    Shape(Shape),
    Pattern(Pattern),
    GameSpecificElement(GameSpecificElement),
}

enum Verb {
    Action(Action),
    LogicalOperation(LogicalOperation),
    SpatialRelation(SpatialRelation),
}

enum ClickAction {
    TransformTo(Noun),
}

struct ElementStyle {
    text: Text, // Bevy Text
    color: Color,
    background_color: Color,
    sprite: String,
    width_percent: f64,
    height_percent: f64,
}

enum SpatialElement {
    Cell {
        name: String,
        rand_args_index: Option<usize>,
        l_click: Option<ClickAction>,
        r_click: Option<ClickAction>,
        style: ElementStyle,
        // is_valid: Option<Logic>,
    },
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
impl SpatialElement {
    fn from_build(builder: SpatialElementBuilder) -> Self {
        match builder {
            SpatialElementBuilder::Cell {
                name,
                rang_args_index,
                l_click,
                r_click,
                style,
                // is_valid,
            } => {
                
            },
        }
    }
}

enum Shape {
    Line,
    Square,
    Rectangle,
    Triangle,
    Circle,
    Cross,
    Star,
    Arrow,
}

enum Pattern {
    Tetromino,
    Pentomino,
}

enum GameSpecificElement {
    Clue,
    Hint,
    Number,
    Symbol,
    Color,
    Shade,
}

enum Action {
    Place,
    Remove,
    Swap,
    Rotate,
    Flip,
    Connect,
    Separate,
    Match,
    Group,
    Divide,
}

enum LogicalOperation {
    Include,
    Exclude,
    Negate,
    Combine,
    Intersect,
    Union,
}

enum SpatialRelation {
    Touch,
    Overlap,
    Surround,
    Isolate,
    Align,
    Bisect,
}