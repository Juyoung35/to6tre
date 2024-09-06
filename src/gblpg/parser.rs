use std::fs::File;
use serde::{Serialize, Deserialize};
use ron;
use super::term_builder::GameBuilder;

fn read_ron(contents: &mut String) -> std::io::Result<()> {
    let file = File::open("games.ron")?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(contents)?;
    Ok(())
}

fn parse_spatial_elements() -> std::io::Result<()> {
    let mut contents = String::new();
    read_ron(&mut contents);
    let games: HashMap<String, GameBuilder> = ron::from_str(contents).unwrap();
    for (game_name, game_elements) in games {
        
    }
    Ok(())
}