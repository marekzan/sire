use std::{error::Error, fs::File, io::Write};

use super::data::{get_data_dir, Data};

pub fn save_data(data: &Data) -> Result<(), Box<dyn Error>> {
    let data_path = get_data_dir()?;
    let data_yaml = serde_yaml::to_string(data)?;
    let mut data_file = File::create(data_path)?;
    data_file.write_all(data_yaml.as_bytes())?;
    Ok(())
}
