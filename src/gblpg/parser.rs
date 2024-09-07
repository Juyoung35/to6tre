use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use ron::{self, extensions::Extensions, options::Options};

use super::term_builder::*;

fn read_ron(contents: &mut String, file_path: &str) -> std::io::Result<()> {
    // let file = File::open("games.ron")?;
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_string(contents)?;
    Ok(())
}

pub fn parse_spatial_elements(file_path: &str) -> std::io::Result<()> {
    // Setup the options
    // let options = Options::default().without_default_extension(Extensions::EXPLICIT_STRUCT_NAMES);
    let options = Options::default()
        .without_default_extension(Extensions::EXPLICIT_STRUCT_NAMES)
        .with_default_extension(Extensions::IMPLICIT_SOME);

    let mut contents = String::new();
    read_ron(&mut contents, file_path);
    let games: HashMap<String, GameBuilder> = options.from_str(&mut contents).unwrap();
    // let games: Vec<Test> = options.from_str(&mut contents).unwrap();
    println!("{games:?}");
    // for (game_name, game_elements) in games {
    //     println!("{game_name:?}\n{game_elements:?}");
    // }
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Test {
    A(usize),
    B { id: usize },
}