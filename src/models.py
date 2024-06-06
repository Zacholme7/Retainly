class Card:
    def __init__(self, card_id, term, definition, level):
        self.id = card_id
        self.term = term
        self.definition = definition
        self.level = level

    def __repr__(self):
        return f'Card(id={self.id}, term={self.term}, definition={self.definition}, level={self.level})'
