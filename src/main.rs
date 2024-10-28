use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
  let path = env::current_dir()?;
  // Get the base config directory (typically ~/.config on Linux)
  if let Some(config_dir) = dirs::config_dir() {
    // Append the application-specific path
    let sire_dir = config_dir.join("sire");
    let config_path = sire_dir.join("config.yaml");

    // Check if the file exists
    if config_path.exists() {
      println!("Config file found at: {:?}", config_path);
    } else {
      if !sire_dir.exists() {
        fs::create_dir_all(&sire_dir).expect("Failed to create config directory");
      }

      // Create the `config.yaml` file with default content
      let mut file = fs::File::create(&config_path).expect("Failed to create config file");
      file
        .write_all(b"/home/mrk/workspace/temp/sire/")
        .expect("Failed to write default config");

      println!("Created default config file at: {:?}", config_path);
    }
  } else {
    println!("Could not determine the config directory.");
  }
  Ok(())
}
