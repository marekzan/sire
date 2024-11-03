use core::{
    db::test::query,
    io::{data::Data, print::print_data},
};
use std::error::Error;

pub fn check(path: &str, data: &Data) -> Result<(), Box<dyn Error>> {
    print_data(path, data)?;
    Ok(())
}

pub fn check_db() -> Result<(), Box<dyn Error>> {
    query()?;
    Ok(())
}
