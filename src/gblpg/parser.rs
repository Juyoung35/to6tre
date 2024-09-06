use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use ron;
use super::term_builder::GameBuilder;

fn read_ron(contents: &mut String, file_path: &str) -> std::io::Result<()> {
    // let file = File::open("games.ron")?;
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(contents)?;
    Ok(())
}

pub fn parse_spatial_elements(file_path: &str) -> std::io::Result<()> {
    let mut contents = String::new();
    read_ron(&mut contents, file_path);
    let games: HashMap<String, GameBuilder> = ron::from_str(&mut contents).unwrap();
    for (game_name, game_elements) in games {
        println!("{game_name:?}\n{game_elements:?}");
    }
    Ok(())
}