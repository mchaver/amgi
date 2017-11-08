// #[macro_use] extern crate text_io;
extern crate rusqlite;
extern crate time;
extern crate quizlib;

use rusqlite::Connection;
use std::io::{BufReader,BufRead};
use std::fs::File;

use rusqlite::types::Value::Null;

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
                  kanji_id        INTEGER NOT NULL,
                  FOREIGN KEY(kanji_id) REFERENCES kanji(id)
                  )", &[]).unwrap();

    conn.execute("CREATE TABLE kunyomi (
                  id              INTEGER PRIMARY KEY,
                  kunyomi         TEXT NOT NULL,
                  okurigana_index INTEGER,
                  kanji_id        INTEGER NOT NULL,
                  FOREIGN KEY(kanji_id) REFERENCES kanji(id)
                  )", &[]).unwrap();
    
    for kanji in kanjis {
        conn.execute("INSERT INTO kanji (kanji) VALUES (?1)", &[&kanji.kanji]).unwrap();
        let kanji_id = conn.last_insert_rowid();
        for onyomi in kanji.onyomis {
            conn.execute("INSERT INTO onyomi (onyomi,kanji_id) VALUES (?1,?2)", &[&onyomi, &kanji_id]).unwrap();
        }
        for kunyomi in kanji.kunyomis {
            let mut oi = 0;
            //"Program".chars().position(|c| c == 'g').unwrap()
            // kunyomi.find('・')
            match kunyomi.chars().position(|c| c == '・') {
                Some(okurigana_index) => {
                    let split_on_dot : Vec<String> = kunyomi.split("・").map(|s| s.to_string()).collect();
                    let without_dot = split_on_dot.join("");
                    oi = okurigana_index as u32;
                    conn.execute("INSERT INTO kunyomi (kunyomi,okurigana_index,kanji_id) VALUES (?1,?2,?3)", &[&without_dot, &oi, &kanji_id]).unwrap();
                },
                None => {
                    conn.execute("INSERT INTO kunyomi (kunyomi,okurigana_index,kanji_id) VALUES (?1,?2,?3)", &[&kunyomi, &Null, &kanji_id]).unwrap();
                },
            }
            // conn.execute("INSERT INTO kunyomi (kunyomi,okurigana_index,kanji_id) VALUES (?1,?2,?3)", &[&kunyomi, &Null, &kanji_id]).unwrap();
            //conn.execute("INSERT INTO kunyomi (kunyomi,kanji_id) VALUES (?1,?2)", &[&kunyomi, &kanji_id]).unwrap();
        }
    }
}

struct KanjiRow {
    id: i32,
    kanji: String,
    onyomi: Option<String>,
    kunyomi: Option<(String,Option<u32>)>
}


fn getKanjis(conn: Connection) {
    let mut stmt = conn.prepare("SELECT kanji.id, kanji.kanji, onyomi.onyomi, kunyomi.kunyomi, kunyomi.okurigana_index FROM kanji LEFT OUTER JOIN onyomi ON kanji.id = onyomi.kanji_id LEFT OUTER JOIN kunyomi ON kanji.id = kunyomi.kanji_id;
").unwrap();

    let kanji_rows = stmt.query_map(&[], |row| {
        KanjiRow {
            id: row.get(0),
            kanji: row.get(1),
            onyomi:
            match row.get_checked(2) {
                Ok(s) => Some(s),
                Err(_) => None,
            },
            kunyomi:
            match row.get_checked(3) {
                Ok(s) => match row.get_checked(4) {
                    Ok(okurigana_index) => Some((s,Some(okurigana_index))),
                    Err(_) => Some((s,None)),
                },
                Err(_) => None,
            },
        }
    }).unwrap();
    
    //for kanji_row in kanji_rows {
    // mutable hashmap of kanji string to kanji
    //}
}


// rusqlite::types::Null
// SELECT kanji.id, kanji.kanji, onyomi.onyomi, kunyomi.kunyomi FROM kanji LEFT OUTER JOIN onyomi ON kanji.id = onyomi.kanji_id LEFT OUTER JOIN kunyomi ON kanji.id = kunyomi.kanji_id;

// SELECT kanji.id, kanji.kanji, onyomi.onyomi, kunyomi.kunyomi FROM kanji LEFT OUTER JOIN onyomi ON kanji.id = onyomi.kanji_id LEFT OUTER JOIN kunyomi ON kanji.id = kunyomi.kanji_id;

// query for kunyomi with okurigana
// SELECT kanji.id, kanji.kanji, kunyomi.kunyomi, kunyomi.okurigana_index FROM kanji LEFT OUTER JOIN kunyomi ON kanji.id = kunyomi.kanji_id WHERE kunyomi.okurigana_index IS NOT NULL;
