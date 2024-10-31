use super::data::Data;

pub fn print_data(path: &str, data: &Data) -> Result<(), Box<dyn std::error::Error>> {
    data.directories.iter().for_each(|dir| {
        if dir.name == path {
            for (index, note) in dir.notes.iter().enumerate() {
                println!("Note #{}", index + 1);
                println!("{}", note.content);
                println!();
            }
        }
    });

    Ok(())
}
