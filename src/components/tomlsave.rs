use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::{collections::BTreeMap, fs::OpenOptions};

use crate::app::App;
use serde::{Deserialize, Serialize};
pub mod system;

#[derive(Debug, Default, Serialize)]
pub struct TabSlots<'a> {
    tabs: BTreeMap<&'a str, TabSlot<'a>>,
}

#[derive(Debug, Serialize)]
struct TabSlot<'a> {
    #[serde(rename = "title")]
    title: &'a str,

    #[serde(rename = "uncoded")]
    uncoded: &'a str,

    #[serde(rename = "encoded")]
    encoded: &'a str,
}

pub const GAMEDATA: &str = "traffic_light_game_data.toml";

pub enum TomlOperations {
    Create,
    Read,
    Update,
    Delete,
}

impl TomlOperations {
    pub fn create(tab_index: &str, title: &str, uncoded: &str, encoded: &str) {
        let mut file = TabSlots::default();

        file.tabs.insert(
            tab_index,
            TabSlot {
                title,
                uncoded,
                encoded,
            },
        );

        let toml_string = toml::to_string(&file).expect("Could not encode TOML Value");

        fs::write(GAMEDATA, toml_string).expect("Could not write to this file ):");
    }

    pub fn read(app: &mut App) -> std::io::Result<()> {
        let mut file = File::open(GAMEDATA)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        app.code = contents;
        Ok(())
    }

    fn delete() -> std::io::Result<()> {
        // Why do we want to delete anything?
        fs::remove_file(GAMEDATA)?;

        Ok(())
    }
}
