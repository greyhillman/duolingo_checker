//! A crate for dealing with Duolingo.
//!
//! This crate provides functions for dealing with extracting
//! Duolingo words from the "Words" page of the site.
//! Getting the contents of the page can be done by selecting
//! all of the text, putting it into a file, and then manually
//! removing extranneous information. The file should be in the
//! form of:
//!
//! ```text,ignore
//! <Word>'\t'<Part of speech>'\t'<Last Practiced>'\t'
//! <Word>'\t'<Part of speech>'\t'<Last Practiced>'\t'
//! ...
//! ```
//!
//! where the '\t' are just TAB characters (I don't know how to
//! get it to just have it without the ').
use std::fmt;
use std::collections::HashSet;

#[macro_use]
extern crate nom;
use nom::IResult;

/// A struct used to hold data from Duolingo.
#[derive(Debug, PartialEq, Clone)]
pub struct DuolingoWord<'a> {
    /// The word itself
    pub word: &'a str,
    /// The type of word. Like Noun or Adjective.
    pub word_class: &'a str,
    /// The last time the word was studied
    pub last_studied: &'a str,
}

impl<'a> fmt::Display for DuolingoWord<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Word: {}\tType: {}\tLast Studied: {}",
            self.word,
            self.word_class,
            self.last_studied
        )
    }
}

/// Extract the list of `DuolingoWord`s from the file contents.
///
/// The content should be in the format as specified in the module
/// docs.
///
/// # Note:
///
/// The `Err` should be done in a better way.
///
/// # Examples:
///
/// ```rust
/// # use duolingo_checker::{DuolingoWord, get_words};
/// let content = "Glas\tNoun\t33 minutes ago\t\nMann\tNoun\t3 months ago\t";
/// let words = get_words(content).unwrap();
///
/// let glas_word = DuolingoWord {
///     word: "Glas",
///     word_class: "Noun",
///     last_studied: "33 minutes ago",
/// };
/// let mann_word = DuolingoWord {
///     word: "Mann",
///     word_class: "Noun",
///     last_studied: "3 months ago",
/// };
///
/// assert_eq!(words.len(), 2);
/// assert_eq!(words[0], glas_word);
/// assert_eq!(words[1], mann_word);
/// ```
pub fn get_words(content: &str) -> Result<Vec<DuolingoWord>, String> {
    named!(
        section<&str>,
        map_res!(take_until_and_consume_s!("\t"), std::str::from_utf8)
    );

    named!(parse<DuolingoWord>, do_parse!(
        word: section >>
        word_class: section >>
        last_studied: section >>
        (DuolingoWord {
            word, word_class, last_studied
        })
    ));

    let mut words = Vec::new();
    for line in content.lines() {
        let word = match parse(line.as_bytes()) {
            IResult::Done(_, word) => word,
            IResult::Error(e) => return Err(format!("{:?}", e)),
            IResult::Incomplete(needed) => return Err(format!("{:?}", needed)),
        };

        words.push(word);
    }

    Ok(words)
}

/// Create a `HashSet` from a list of list of words.
///
/// # Examples:
/// ```rust
/// # use duolingo_checker::build_word_map;
/// let words = vec![vec!["a", "b", "c"],
///                  vec!["d", "e"],
///                  vec!["a"]];
/// let set = build_word_map(&words);
///
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(set.contains("e"));
/// assert!(!set.contains("f"));
/// ```
pub fn build_word_map<'a>(words: &Vec<Vec<&'a str>>) -> HashSet<&'a str> {
    let mut map = HashSet::new();

    for fields in words {
        for field in fields {
            map.insert(*field);
        }
    }

    map
}

/// Extract the fields from an Anki note.
///
/// Anki uses the special symbol '\x1f' to separate the fields of
/// a note.
///
/// # Examples:
///
/// ```rust
/// # use duolingo_checker::to_fields;
/// assert_eq!(to_fields("a\x1fb\x1fc"), vec!["a", "b", "c"]);
/// ```
pub fn to_fields(fields: &str) -> Vec<&str> {
    fields.split('\x1f').collect()
}

/// Extract the fields from the list of fields.
///
/// # Examples:
/// ```rust
/// # use duolingo_checker::get_words_from_fields;
/// let fields = vec![format!("a\x1fb\x1fc"), format!("d\x1fe"), format!("a")];
/// let true_fields = vec![vec!["a", "b", "c"],
///                        vec!["d", "e"],
///                        vec!["a"]];
/// assert_eq!(get_words_from_fields(&fields), true_fields);
/// ```
pub fn get_words_from_fields<'a>(fields: &'a Vec<String>) -> Vec<Vec<&'a str>> {
    fields
        .iter()
        .map(|x| to_fields(x))
        .collect::<Vec<Vec<&str>>>()
}
