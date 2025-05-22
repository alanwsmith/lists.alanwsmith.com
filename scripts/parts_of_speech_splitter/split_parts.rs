---
[dependencies]
anyhow = "1.0.98"
serde_json = "1.0.140"
---

use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use serde_json::{Value};
use std::collections::BTreeSet;

struct Book {
    letters: Vec<Letter>
}

struct Letter {
    words: Value
}

impl Book {
    pub fn get_word_kinds(&self) -> Result<BTreeSet<String>> {
        let mut kinds: BTreeSet<String> = BTreeSet::new();
        let kind = "adjective";
        self.letters.iter().for_each(|letter| letter.words
            .as_object()
            .expect("asdf")
            .values()
            .for_each(|x| {
                    match x["meanings"].as_array() {
                        Some(meanings) => { 
                            meanings.iter().for_each(|meaning| {
                                match meaning["speech_part"].as_str() {
                                    Some(speech_part) => {
                                        kinds.insert(speech_part.to_string());
                                        // if kind == speech_part {
                                        //     Some(x["word"].as_str()?.to_string())
                                        // } else {
                                        //     None
                                        // }

                                    },
                                None => ()
                                }
                            })
                        }
                                None => ()
                    }
            })
        );
        Ok(kinds)
    }

    pub fn get_words_of_kind(&self, kind: &str) -> Result<Vec<String>> {
        let mut words: Vec<String> = self.letters.iter().flat_map(|letter| letter.words
            .as_object()
            .expect("asdf")
            .values()
            .filter_map(|x| {
                if no_spaces_etc(x["word"].as_str().unwrap()) {
                    match x["meanings"].as_array() {
                        Some(meanings) => { 
                            meanings.iter().find_map(|meaning| {
                                match meaning["speech_part"].as_str() {
                                    Some(speech_part) => {
                                        if kind == speech_part {
                                            Some(x["word"].as_str()?.to_string())
                                        } else {
                                            None
                                        }
                                    },
                                    None => None
                                }
                            })
                        }
                        None => None
                    }
                } else {
                    None
                }
            }).collect::<Vec<String>>()
        ).collect();
        words.sort();
        Ok(words)
    }
}

fn no_spaces_etc(check: &str) -> bool {
    // NOTE: was doing this to remove some stuff
    // from adjectives. Since all words types
    // are going now, I'm leaving everything in 
    // and just relying on filtering downstream if 
    // it's necessary.
    // check.chars().nth(0).expect("getting_char").is_lowercase()
    //     && 
    // !check.contains(" ")
    //     && 
    // !check.contains(".")
    //     && 
    // !check.contains("-")
    true
}

fn main() -> Result<()> {
    let book = load_data()?;
    let word_kinds = book.get_word_kinds();
    dbg!(word_kinds);
    let word_types = vec![
        ("adjective", "adjectives"),
        ("adverb", "adverbs"),
        ("article", "articles"),
        ("interjection", "interjections"),
        ("noun", "nouns"),
        ("pronoun", "pronouns"),
        ("verb", "verbs"),
    ];
    make_files(&book, &word_types)?;
    Ok(())
}

pub fn make_files(book: &Book, word_types: &Vec<(&str, &str)>) -> Result<()> {
    word_types.iter().for_each(|wt| {
        let output_path = PathBuf::from(format!("../../docs/txt/english-{}.txt", wt.1));
        let words = book.get_words_of_kind(wt.0).unwrap();
        fs::write(output_path, words.join("\n")).unwrap();
        let about_path = PathBuf::from(format!("../../docs/txt/english-{}-about.txt", wt.1));
        let about_txt = format!("A collection of {} from the open source Wordset Dictionary\nhttps://github.com/wordset/wordset-dictionary", wt.1);
        fs::write(about_path, about_txt).unwrap();
    });
    Ok(())
}

pub fn load_data() -> Result<Book> {
    let files = get_files_in_dir(&PathBuf::from("data-letters"))?;
    let book = Book {letters: files.iter().map(|f| {
        let content = fs::read_to_string(f).unwrap();
        let letter = 
            Letter{
            words: serde_json::from_str(&content).unwrap()
            };
        letter
    }).collect()};
    Ok(book)
}

pub fn get_files_in_dir(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let files = fs::read_dir(dir)?
        .into_iter()
        .filter(|p| {
            if p.as_ref().unwrap().path().is_file() {
                true
            } else {
                false
            }
        })
        .map(|p| p.as_ref().unwrap().path())
        .filter(|p| {
            !p.file_name().unwrap().to_str().unwrap().starts_with(".")
        })
        .collect();
    Ok(files)
}
