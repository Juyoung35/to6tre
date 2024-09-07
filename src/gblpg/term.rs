use super::term::*;

pub(super) enum Noun {
    SpatialElement(SpatialElement),
    Shape(Shape),
    Pattern(Pattern),
    GameSpecificElement(GameSpecificElement),
}

pub(super) enum Verb {
    Action(Action),
    LogicalOperation(LogicalOperation),
    SpatialRelation(SpatialRelation),
}

pub(super) enum ClickAction {
    TransformTo(Noun),
}

pub(super) struct ElementStyle {
    text: Text, // Bevy Text
    color: Color,
    background_color: Color,
    sprite: String,
    width_percent: f64,
    height_percent: f64,
}

pub(super) enum SpatialElement {
    Cell {
        name: &str,
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
    fn from_build(builder: SpatialElementBuilder, noun_vocab: &Vec<Noun>) -> Self {
        match builder {
            SpatialElementBuilder::Cell {
                name,
                random,
                l_click,
                r_click,
                style,
                // is_valid,
            } => {

                Self {
                    name,
                }
            },
            _ => panic!(),
        }
    }
}

pub(super) enum Shape {
    Line,
    Square,
    Rectangle,
    Triangle,
    Circle,
    Cross,
    Star,
    Arrow,
}

pub(super) enum Pattern {
    Tetromino,
    Pentomino,
}

pub(super) enum GameSpecificElement {
    Clue,
    Hint,
    Number,
    Symbol,
    Color,
    Shade,
}

pub(super) enum Action {
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

pub(super)enum LogicalOperation {
    Include,
    Exclude,
    Negate,
    Combine,
    Intersect,
    Union,
}

pub(super) enum SpatialRelation {
    Touch,
    Overlap,
    Surround,
    Isolate,
    Align,
    Bisect,
}