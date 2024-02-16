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

pub fn query_cards(conn: &Connection) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT term, definition FROM card")?;
    let card_iter = stmt.query_map([], |row| {
        Ok(Card {
            term: row.get(0)?,
            definition: row.get(1)?,
        })
    })?;

    let mut cards = Vec::new();
    for card in card_iter {
        cards.push(card?);
    }

    Ok(cards)
}
