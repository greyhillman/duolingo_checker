import argparse
import sys

import sqlite3 as sql
from typing import List, Tuple


def get_words(fields: List[str], parts_of_speech: List[str]) -> List[Tuple[str, str]]:
    words = []
    for line in sys.stdin:
        if not line:
            continue

        word, part, last, __ = line.split('\t')

        if parts_of_speech:
            if part in parts_of_speech and word not in fields:
                words.append((word, part))
        else:
            if word not in fields:
                words.append((word, part))

    return words


def get_fields(database_file: str) -> List[str]:
    with sql.connect(database_file) as connection:
        cursor = connection.cursor()
        cursor.execute('SELECT sfld from notes')

        cursor.execute('SELECT flds from notes')
        fields = [field[0].split('\x1f') for field in cursor.fetchall()]
        fields = [f for field in fields for f in field if f and len(f) <= 30]

        return fields


def parse_args():
    parser = argparse.ArgumentParser()

    parser.add_argument('anki_database', help='The database file for Anki')
    parser.add_argument('parts_of_speech', nargs='*', help='the parts of speech to get', default=[])

    return parser.parse_args()


def main():
    arguments = parse_args()
    database_file = arguments.anki_database
    parts_of_speech = arguments.parts_of_speech

    fields = get_fields(database_file)

    words = get_words(fields, parts_of_speech)
    for word, part in words:
        print(f'{word} - {part}')

    print(len(words))


if __name__ == '__main__':
    main()
