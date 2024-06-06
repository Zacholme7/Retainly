from db_operations import load_cards, add_card, update_card, delete_card, get_current_day, set_current_day
from models import Card
from openai import OpenAI

client = OpenAI(api_key="your private key")


def batch_add_cards():
    print("Enter your data. Type 'done' on a new line when finished.")
    user_input = []
    while True:
        line = input()
        if line.strip().lower() == "done":
            break
        user_input.append(line)

    content = "\n".join(user_input)

    messages = [
        {
            "role": "system",
            "content": "Please extract the following terms and definitions and return them in the format 'term: definition.' for each line. Keep each term:definition on one line. Do not include leading '-' and some definitions may contains delimiting characters such as ':', '->', etc"
        },
        {
            "role": "user",
            "content": f"{content}"
        }
    ]

    response = client.chat.completions.create(model="gpt-4o", messages=messages, max_tokens=4000)
    response = response.choices[0].message.content.strip()

    cards = parse_response(response)

    print("\nParsed Cards:")
    for idx, (term, definition) in enumerate(cards, 1):
        print(f"{idx}) {term}: {definition}")

    confirmation = input("\nAre these cards correct? (y/n): ").strip().lower()
    if confirmation == "y":
        for term, definition in cards:
            add_card_command(term, definition)
        print("All cards have been added.")
    else:
        print("Batch addition aborted.")


def parse_response(response):
    lines = response.split("\n")
    cards = []
    for line in lines:
        if ":" in line:
            term, definition = line.split(":", 1)
            cards.append((term.strip(), definition.strip()))
    return cards


def list_cards():
    cards = load_cards()
    levels = {i: [] for i in range(1, 8)}
    for card in cards:
        levels[card.level].append(card)

    for level in range(1, 8):
        print(f"\n{'='*10} Level {level} {'='*10}")
        for idx, card in enumerate(levels[level], 1):
            print(f'{idx}) {card.term}: {card.definition}')


def list_all_cards_sequentially():
    cards = load_cards()
    for idx, card in enumerate(cards, 1):
        print(f'{idx}) {card.term}: {card.definition}')


def get_card_by_display_id(display_id):
    cards = load_cards()
    if 1 <= display_id <= len(cards):
        return cards[display_id - 1]
    return None


def modify_card(display_id, term, definition):
    card = get_card_by_display_id(display_id)
    if card:
        card.term = term
        card.definition = definition
        update_card(card)
        print(f'Card {display_id} updated.')
    else:
        print(f'Card {display_id} not found.')


def delete_card_command(display_id):
    card = get_card_by_display_id(display_id)
    if card:
        delete_card(card.id)
        print(f'Card {display_id} deleted.')
    else:
        print(f'Card {display_id} not found.')


def add_card_command(term, definition):
    new_card = Card(None, term, definition, 1)
    add_card(new_card)
    print(f'Card added with term: {term} and definition: {definition}.')


def start_day():
    current_day = get_current_day()
    levels_to_review = SCHEDULE[current_day - 1]["levels"]

    print(f"Day {current_day}")
    print(f"Levels to review: {levels_to_review}")
    print("----------------------------------------")

    cards = load_cards()
    day_cards = [card for card in cards if card.level in levels_to_review]
    day_cards.sort(key=lambda x: x.level)

    while day_cards:
        card = day_cards.pop(0)
        print(f'Term: {card.term}, Level: {card.level}')
        action = input("Options: show/y/n/exit: ").strip().lower()

        if action == "show":
            print(f'Definition: {card.definition}')
            action = input("Options: y/n: ").strip().lower()

        if action == "y":
            card.level = min(card.level + 1, 7)
            update_card(card)
            print(f'Card {card.term} incremented to level {card.level}.')
        elif action == "n":
            card.level = 1
            update_card(card)
            day_cards.append(card)
            print(f'Card {card.term} reset to level {card.level}.')
        elif action == "exit":
            print("Exiting review for the day.")
            day_cards.insert(0, card)  # Put the current card back in the list
            break

    if not day_cards:
        current_day = (current_day % len(SCHEDULE)) + 1
        set_current_day(current_day)
        print(f'Day {current_day} completed. Moving to day {current_day}.')


# Schedule for all days
SCHEDULE = [
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [4, 1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [5, 1]},
    {"levels": [4, 2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [4, 1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [6, 1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [5, 1]},
    {"levels": [4, 2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [4, 1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [5, 1]},
    {"levels": [4, 2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [4, 1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [7, 1]},
    {"levels": [2, 1]},
    {"levels": [3, 1]},
    {"levels": [6, 2, 1]},
    {"levels": [5, 1]},
    {"levels": [4, 2, 1]},
    {"levels": [3, 1]},
    {"levels": [2, 1]},
    {"levels": [1]},
]
