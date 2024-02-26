use common::Card;
use rusqlite::Connection;

/// Create a table in the database to hold the cards
pub fn create_table(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS card (
            id INTEGER PRIMARY KEY,
            term TEXT NOT NULL,
            definition TEXT NOT NULL,
            level INTEGER NOT NULL
         )",
        [],
    )?;
    Ok(())
}

/// Insert a card into the database
pub fn insert_card_into_db(
    conn: &Connection,
    card: &Card,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO card (term, definition, level) VALUES (?1, ?2, ?3)",
        &[
            &card.term,
            &card.definition,
            &card.current_level.to_string(),
        ],
    )?;
    Ok(())
}

/// Get a list of all the cards in the database
pub fn query_cards(conn: &Connection) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, term, definition, level FROM card")?;
    let card_iter = stmt.query_map([], |row| {
        Ok(Card {
            id: row.get(0)?,
            term: row.get(1)?,
            definition: row.get(2)?,
            current_level: row.get(3)?,
        })
    })?;

    let mut cards = Vec::new();
    for card in card_iter {
        cards.push(card?);
    }

    Ok(cards)
}

/// Get card by ID
pub fn get_card(conn: &Connection, id: i64) -> Result<Card, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, term, definition, level FROM card WHERE id = ?1")?;
    stmt.query_row([id], |row| {
        Ok(Card {
            id: row.get(0)?,
            term: row.get(1)?,
            definition: row.get(2)?,
            current_level: row.get(3)?,
        })
    })
    .map_err(|e| e.into())
}

/// Remove a card from the database by ID
pub fn remove_card_from_db(conn: &Connection, id: i64) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute("DELETE FROM card WHERE id = ?1", [id])?;
    Ok(())
}

/// Move a card up a level
pub fn move_card_up_level(conn: &Connection, id: i64) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute("UPDATE card SET level = level + 1 WHERE id = ?1", [id])?;
    Ok(())
}

/// Move card back to level one
pub fn move_card_to_level_one(
    conn: &Connection,
    id: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute("UPDATE card SET level = 1 WHERE id = ?1", [id])?;
    Ok(())
}
