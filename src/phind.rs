use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import necessary types from your main crate
use crate::components::{SpatialElement, Color};

#[derive(Serialize, Deserialize)]
pub struct GameConfigs<'a> {
    pub games: HashMap<&'a str, GameConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct GameConfig {
    pub nouns: Nouns,
}

#[derive(Serialize, Deserialize)]
pub struct Nouns<'a> {
    pub spatial_elements: HashMap<&'a str, SpatialElement>,
}

#[derive(Serialize, Deserialize)]
pub enum SpatialElementBuilder {
    Cell {
        color: ColorBuilder,
        probability: Option<f32>,
    },
}

struct ColorBuilder<'a>(&'a str);

impl<'a> GameConfigs<'a> {
    pub fn parse<R: std::io::Read>(reader: R) -> Result<Self, serde_json::Error> {
        let mut configs = String::new();
        reader.read_to_string(&mut configs)?;
        
        serde_json::from_str(&configs)
    }

    pub fn generate_grid(&self, game_name: &str, width: u32, height: u32) -> Vec<Vec<(NounID, Entity)>> {
        let config = self.games.get(game_name).expect("Game not found");
        let mut grid = vec![vec![(NounID(0), Entity::default()); width as usize]; height as usize];

        for x in 0..width {
            for y in 0..height {
                if let Some(element) = config.nouns.spatial_elements.get("cell") {
                    let rng = rand::thread_rng();
                    if rng.gen::<f32>() < element.color.probability {
                        let cell_entity = Self::create_cell_entity(x, y, &element.color);
                        grid[x as usize][y as usize] = (NounID(0), cell_entity);
                    }
                }
            }
        }

        grid
    }

    fn create_cell_entity(x: u32, y: u32, color: &Color) -> Entity {
        let mesh = Mesh::from(Icosphere::default());
        let material = SimpleMaterial::new(color.clone(), Color::WHITE);
        let transform = Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0));

        let mut commands = Commands::default();
        commands
            .insert(NodeBundle {
                mesh: Some(mesh),
                material: Some(material),
                transform: Some(transform),
                ..Default::default()
            })
            .insert(SpatialElement::Cell { color: color.clone(), probability: 1.0 });

        commands.entity
    }
}

pub fn parse_game_configs<R: std::io::Read>(reader: R) -> Result<GameConfigs, serde_json::Error> {
    GameConfigs::parse(reader)
}
