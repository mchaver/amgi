// #[macro_use] extern crate text_io;
extern crate rusqlite;
extern crate time;
extern crate quizlib;

use rusqlite::Connection;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {
    let file = File::open("pronunciation.txt").unwrap();
    let mut kanjis : Vec<quizlib::Kanji> = vec![];
    for line in BufReader::new(file).lines() {
        let items =  line.unwrap();
        let v: Vec<&str> = items.split("|").collect();
        let onyomis: Vec<String> = v[1].split(",").map(|s| s.to_string()).filter(|s| s != "").collect();
        let kunyomis: Vec<String> = v[2].split(",").map(|s| s.to_string()).filter(|s| s != "").collect();        
        kanjis.push(quizlib::Kanji{kanji: v[0].to_owned(), onyomis: onyomis.to_owned(), kunyomis: kunyomis.to_owned()});
    }

    let conn = Connection::open("test.sql").unwrap();

    conn.execute("CREATE TABLE kanji (
                  id              INTEGER PRIMARY KEY,
                  kanji           TEXT NOT NULL
                  )", &[]).unwrap();

    conn.execute("CREATE TABLE onyomi (
                  id              INTEGER PRIMARY KEY,
                  onyomi          TEXT NOT NULL,
                  kanjiid         INTEGER,
                  FOREIGN KEY(kanjiid) REFERENCES kanji(id)
                  )", &[]).unwrap();

    conn.execute("CREATE TABLE kunyomi (
                  id              INTEGER PRIMARY KEY,
                  kunyomi          TEXT NOT NULL,
                  kanjiid         INTEGER,
                  FOREIGN KEY(kanjiid) REFERENCES kanji(id)
                  )", &[]).unwrap();
    
    for kanji in kanjis {
        conn.execute("INSERT INTO kanji (kanji) VALUES (?1)", &[&kanji.kanji]).unwrap();
        let kanjiid = conn.last_insert_rowid();
        for onyomi in kanji.onyomis {
            conn.execute("INSERT INTO onyomi (onyomi,kanjiid) VALUES (?1,?2)", &[&onyomi, &kanjiid]).unwrap();
        }
        for kunyomi in kanji.kunyomis {
            conn.execute("INSERT INTO kunyomi (kunyomi,kanjiid) VALUES (?1,?2)", &[&kunyomi, &kanjiid]).unwrap();
        }
    }

    //let s: String = read!("{}");
    //println!("{}", s);
}
