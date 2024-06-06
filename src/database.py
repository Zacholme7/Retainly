import sqlite3
from contextlib import closing

DATABASE_FILE = 'spaced_repetition.db'


def initialize_database():
    with closing(sqlite3.connect(DATABASE_FILE)) as conn:
        with conn:
            conn.execute('''
                CREATE TABLE IF NOT EXISTS day (
                    id INTEGER PRIMARY KEY,
                    current_day INTEGER NOT NULL
                )
            ''')
            conn.execute('''
                CREATE TABLE IF NOT EXISTS card (
                    id INTEGER PRIMARY KEY,
                    term TEXT NOT NULL,
                    definition TEXT NOT NULL,
                    level INTEGER NOT NULL
                )
            ''')


def get_connection():
    return sqlite3.connect(DATABASE_FILE)
