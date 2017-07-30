#[macro_use]
extern crate clap;
use clap::{App, ArgMatches};

extern crate rusqlite;
use rusqlite::Connection;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

extern crate duolingo_checker;
use duolingo_checker::{build_word_map, get_words, get_words_from_fields};


fn get_db_path<'a>(arguments: &'a ArgMatches) -> &'a Path {
    match arguments.value_of("anki_db") {
        Some(path) => Path::new(path),
        None => unreachable!(),
    }
}

fn get_duolingo_file(arguments: &ArgMatches) -> Option<File> {
    match arguments.value_of("duolingo_file") {
        Some(x) => {
            let path = Path::new(x);
            let file = File::open(&path).unwrap();

            Some(file)
        }
        None => None,
    }
}

fn get_contents<S: Read>(mut source: S) -> String {
    let mut contents = String::new();

    match source.read_to_string(&mut contents) {
        Ok(_) => contents,
        _ => panic!(),
    }
}

fn get_fields_from_anki(anki_db: &Path) -> Vec<String> {
    let connection = match Connection::open(anki_db) {
        Ok(conn) => conn,
        Err(why) => panic!(why),
    };

    let mut stmt = connection.prepare("SELECT flds FROM notes").unwrap();
    let stmt_iter = stmt.query_map(&[], |row| {
        let flds: String = row.get(0);

        flds
    }).unwrap();

    let mut fields = Vec::new();
    for thing in stmt_iter {
        fields.push(thing.expect("Failed to get fields from anki"));
    }

    fields
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let arguments = App::from(yaml).get_matches();

    let anki_db = get_db_path(&arguments);
    let duolingo_file = get_duolingo_file(&arguments);

    let duolingo_contents = match duolingo_file {
        Some(file) => get_contents(file),
        None => get_contents(io::stdin()),
    };

    let duolingo_words = match get_words(&duolingo_contents) {
        Ok(words) => words,
        Err(reason) => panic!(reason),
    };

    let anki_fields = get_fields_from_anki(anki_db);
    let anki_words = get_words_from_fields(&anki_fields);

    let word_map = build_word_map(&anki_words);
    let mut num = 0;
    for word in duolingo_words {
        if !word_map.contains(word.word) {
            println!("{}", word);
            num += 1;
        }
    }
    println!("{}", num);
}
