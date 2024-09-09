use bevy::prelude::*;

#[derive(Resource)]
struct GameConfig<'a> {
    name: &'a str,
    nouns: Vec<Noun>,
    grid: Grid,
}

enum Noun {
    SpatialElement(SpatialElement),
}

enum SpatialElement {
    Cell,
}

struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<Entity>>,
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn new_cell_archetype() {

}