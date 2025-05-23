#!/usr/bin/env cargo -Zscript

---
[dependencies]
anyhow = "1.0.98"
minijinja = { version = "2.9.0", features = ["custom_syntax", "loader"] }
---

use anyhow::Result;
use minijinja::{Environment, Value, context};
use minijinja::syntax::SyntaxConfig;
use std::fs;
use std::path::PathBuf;


#[derive(Debug)]
struct FileList {
    paths: Vec<PathBuf>
}

impl FileList {
    pub fn load() -> FileList {
        let dir = PathBuf::from("../../docs/txt");
        let ext = "txt";
        FileList {
            paths: get_files_with_ext(&dir, ext)
        }
    }

    // Ignoring __about.txt files for now.
    // Will add them later
    pub fn txt_files(&self) -> Vec<String> {
        let mut files = self.paths.iter().filter(|p| {
            !p.to_str().unwrap().ends_with("__about.txt")
        }).filter_map(|p| {
                if let Ok(new_path) = p.strip_prefix("../../docs") {
                    Some(new_path.display().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        files.sort();
        files
    }

}

fn main() -> Result<()> {
    let home_page_path = PathBuf::from("../../docs/index.html");
    let fl = FileList::load();
    let mut env = get_env();
    env.add_template_owned("template", make_template())?;
    let jinja = env.get_template("template")?;
    let output = jinja.render(context!(
        file_list => Value::from(fl.txt_files())
    ))?;
    fs::write(home_page_path, output)?;
    Ok(())
}

fn do_output() -> Result<()> {
    Ok(())
}

fn get_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_syntax(
        SyntaxConfig::builder()
            .block_delimiters("[!", "!]")
            .variable_delimiters("[@", "@]")
            .comment_delimiters("[#", "#]")
            .build()
            .unwrap(),
    );
    env
}

fn make_template() -> String {
r#"<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Alan's Lists</title>
    <style>
    a {
        color: slategray;
        text-decoration: none;
    }
    body {
      background-color: black;
      color: goldenrod;
    }
    </style>
  </head>
  <body>
    <h1>Alan's Lists</h1>
    <p>A place for me to collect lists</p>
    <ul>
        [!- for file in file_list -!]
<li><a href="/[@ file @]">[@ file|split("/")|last @]</a></li>
        [! endfor -!]
    </ul>
  </body>
</html>"#.to_string()
}


pub fn get_files_with_ext(dir: &PathBuf, extension: &str) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter(|p| {
            if p.as_ref().unwrap().path().is_file() {
                true
            } else {
                false
            }
        })
        .filter(|p| {
          match p.as_ref().unwrap().path().extension() {
            Some(ext) => ext == extension,
            None => false
          }
        })
        .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
            Ok(_) => None,
            Err(_) => Some(p.as_ref().unwrap().path()),
        })
        .collect()
}


