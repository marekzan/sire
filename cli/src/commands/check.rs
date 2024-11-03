use core::io::{data::Data, print::print_data};

pub fn check(path: &str, data: &Data) -> Result<(), Box<dyn std::error::Error>> {
    print_data(path, data)?;
    Ok(())
}
