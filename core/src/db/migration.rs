use std::error::Error;

use rusqlite::Connection;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/db/migrations");
}

pub fn start_migration() -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open("my_database.db")?;
    embedded::migrations::runner().run(&mut conn)?;
    Ok(())
}
