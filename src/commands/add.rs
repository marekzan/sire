use crate::io::{data::Data, save::save_data};

pub fn add(
    path: &str,
    data: &mut Data,
    content: String,
    detached: &bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if *detached {
        data.add_note(String::from("*global*"), content);
    } else {
        data.add_note(path.to_owned(), content);
    }

    save_data(data)?;

    Ok(())
}
