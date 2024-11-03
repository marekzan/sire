use rusqlite::{params, Connection, Result};

fn create() -> Result<Connection> {
    // Connect to SQLite database (or create it if it doesn't exist)
    let conn = Connection::open("my_database.db")?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER
        )",
        [],
    )?;
    Ok(conn)
}

pub fn insert(name: &str, age: &i32, email: &str) -> Result<()> {
    let conn = create()?;
    // Insert a row into the table
    conn.execute(
        "INSERT INTO person (name, age, email) VALUES (?1, ?2, ?3)",
        params![name, age, email],
    )?;
    Ok(())
}

pub fn query() -> Result<()> {
    let conn = create()?;
    // Query the data
    let mut stmt = conn.prepare("SELECT id, name, age, email FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
            email: row.get(3)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person);
    }

    Ok(())
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    email: Option<String>,
}
