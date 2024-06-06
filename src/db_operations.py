from models import Card
from database import get_connection


def get_current_day():
    with get_connection() as conn:
        cursor = conn.execute('SELECT current_day FROM day WHERE id = 1')
        row = cursor.fetchone()
        return row[0] if row else 1


def set_current_day(day):
    with get_connection() as conn:
        conn.execute('INSERT OR REPLACE INTO day (id, current_day) VALUES (1, ?)', (day,))


def load_cards():
    with get_connection() as conn:
        cursor = conn.execute('SELECT id, term, definition, level FROM card')
        cards = [Card(row[0], row[1], row[2], row[3]) for row in cursor.fetchall()]
    return cards


def add_card(card):
    with get_connection() as conn:
        conn.execute('INSERT INTO card (term, definition, level) VALUES (?, ?, ?)', (card.term, card.definition, card.level))


def update_card(card):
    with get_connection() as conn:
        conn.execute('UPDATE card SET term = ?, definition = ?, level = ? WHERE id = ?', (card.term, card.definition, card.level, card.id))


def delete_card(card_id):
    with get_connection() as conn:
        conn.execute('DELETE FROM card WHERE id = ?', (card_id,))
