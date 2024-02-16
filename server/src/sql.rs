use common::Card;
use rusqlite::Connection;
pub fn create_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS card (
            id INTEGER PRIMARY KEY,
            term TEXT NOT NULL,
            definition TEXT NOT NULL
         )",
        [],
    )?;
    Ok(())
}

pub fn insert_card(conn: &Connection, card: &Card) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO card (term, definition) VALUES (?1, ?2)",
        &[&card.term, &card.definition],
    )?;
    Ok(())
}
