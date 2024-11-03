use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Error},
    path::PathBuf,
};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub directories: Vec<Directory>,
}

impl Data {
    pub fn add_note(&mut self, directory_name: String, note: String) {
        let directory = self
            .directories
            .iter_mut()
            .find(|d| d.name == directory_name);
        match directory {
            Some(directory) => directory.notes.push(Note::new(note)),
            None => self.directories.push(Directory {
                name: directory_name,
                notes: vec![Note::new(note)],
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Directory {
    pub name: String,
    pub notes: Vec<Note>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: String,
    pub content: String,
}

impl Note {
    fn new(note: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            content: note,
        }
    }
}

pub fn load_data() -> Result<Data, Box<dyn std::error::Error>> {
    let data_path = get_data_dir()?;
    let yaml_data = fs::read_to_string(data_path)?;
    let data: Data = serde_yaml::from_str::<Data>(&yaml_data)?;
    Ok(data)
}

pub fn get_data_dir() -> Result<PathBuf, Error> {
    let Some(data_dir) = dirs::data_dir() else {
        return Err(Error::new(
            io::ErrorKind::NotFound,
            "Failed to find config directory",
        ));
    };

    let sire_dir = data_dir.join("sire");
    let data_path = sire_dir.join("data.yaml");

    if !data_path.exists() {
        fs::create_dir_all(sire_dir)?;
        fs::write(&data_path, "")?;
    }

    Ok(data_path)
}
