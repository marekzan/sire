use serde::Deserialize;
use std::{
    env, fs,
    io::{self, Error},
    path::PathBuf,
};

#[derive(Debug, Deserialize)]
struct Root {
    directories: Vec<Directory>,
}

#[derive(Debug, Deserialize)]
struct Directory {
    name: String,
    notes: Vec<Note>,
}

#[derive(Debug, Deserialize)]
struct Note {
    content: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;

    let data = load_data()?;

    data.directories.iter().for_each(|dir| {
        if dir.name == path.to_str().unwrap() {
            for (index, note) in dir.notes.iter().enumerate() {
                println!("Note #{}", index + 1);
                println!("Content: {}", note.content);
                println!();
            }
        }
    });

    Ok(())
}

fn load_data() -> Result<Root, Box<dyn std::error::Error>> {
    let data_path = get_data_dir()?;
    let yaml_data = fs::read_to_string(data_path)?;
    let data: Root = serde_yaml::from_str::<Root>(&yaml_data)?;
    Ok(data)
}

fn get_data_dir() -> Result<PathBuf, Error> {
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
