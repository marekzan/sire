use core::{
    db::test::insert,
    io::{data::Data, save::save_data},
};
use std::error::Error;

pub fn add(
    path: &str,
    data: &mut Data,
    content: String,
    detached: &bool,
) -> Result<(), Box<dyn Error>> {
    if *detached {
        data.add_note(String::from("*global*"), content);
    } else {
        data.add_note(path.to_owned(), content);
    }

    save_data(data)?;

    Ok(())
}

pub fn add_db(name: &str, age: &i32, email: &str) -> Result<(), Box<dyn Error>> {
    insert(name, age, email)?;
    Ok(())
}
