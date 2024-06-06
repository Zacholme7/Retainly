import sys
from commands import list_cards, list_all_cards_sequentially, modify_card, delete_card_command, add_card_command, start_day, batch_add_cards
from database import initialize_database


def print_help():
    print("""
Available commands:
    list                 - List all cards by level
    modify               - Modify a card's term and definition
    delete               - Delete a card by its display index
    add                  - Add a new card with term and definition
    start                - Start the review for the current day
    batch                - Add multiple cards at once
    help                 - Show this help message
    exit                 - Exit the application
""")


def main():
    initialize_database()
    print("Welcome to Retainly")
    print("-------------------")
    print_help()

    while True:
        command = input("\nEnter command: ").strip()

        if not command:
            continue

        if command == "list":
            list_cards()
        elif command == "modify":
            list_all_cards_sequentially()
            try:
                print('\n')
                display_id = int(input("Enter the display ID of the card to modify: ").strip())
                term = input("Enter the new term: ").strip()
                definition = input("Enter the new definition: ").strip()
                modify_card(display_id, term, definition)
            except ValueError:
                print("Invalid input. Display ID must be a number.")
                continue
        elif command == "delete":
            list_all_cards_sequentially()
            print('\n')
            try:
                display_id = int(input("Enter the display ID of the card to delete: ").strip())
                delete_card_command(display_id)
            except ValueError:
                print("Invalid input. Display ID must be a number.")
                continue
        elif command == "add":
            term = input("Enter the term: ").strip()
            definition = input("Enter the definition: ").strip()
            add_card_command(term, definition)
        elif command == "start":
            start_day()
        elif command == "batch":
            print('\n')
            batch_add_cards()
        elif command == "help":
            print_help()
        elif command == "exit":
            print("Exiting application.")
            break
        else:
            print(f"Unknown command: {command}")
            print_help()


if __name__ == "__main__":
    main()
