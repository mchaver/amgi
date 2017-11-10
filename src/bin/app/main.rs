//extern crate rusqlite;
#[macro_use] extern crate text_io;
extern crate quizlib;
extern crate term;

extern crate rand;
extern crate rusqlite;

use rand::thread_rng;
use rand::Rng;
use rusqlite::Connection;

use std::collections::HashMap;

fn shuffle_copy<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_vec();
    let mut rng = thread_rng();
    rng.shuffle(&mut newvec);
    newvec
}
//use rusqlite::Connection;
fn main() {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::BRIGHT_GREEN).unwrap();
    write!(t, "hello, ").unwrap();

    t.fg(term::color::BRIGHT_BLUE).unwrap();
    writeln!(t, "world!").unwrap();

    t.reset().unwrap();

    let conn = Connection::open("test.sql").unwrap();
    let kanjis = get_kanjis(conn);
//    let ks = get_kanji_with_okurigana(conn);

    println!("{:?}", kanjis);

    for kanji in kanjis {
        learn_kanji(&kanji);
    }
//    println!("{:?}", ks);
/*    
    let mkanjis = quizlib::get_kanji("pronunciation.txt");
    let kanjis = mkanjis.unwrap();

    let ks = shuffle_copy(&kanjis);
    for k in ks.iter() {
        quiz_kanji(k);
    }
*/
}

fn learn_kanji(kanji: &quizlib::Kanji) {
    println!("{} onyomi: ", kanji.kanji);
    
    for onyomi in kanji.onyomis.iter() {
        println!("{}", onyomi);
        let mut correct = 0;
        while correct < 5 {
            let onyomi_read: String = read!("{}\n");
            if quizlib::romaji_to_katakana(onyomi_read.trim()) == onyomi.to_string() {
                println!("✓");
                correct += 1;
            } else {
                println!("×");
            }
        }
    }

}

fn print_kanji(kanji: &quizlib::Kanji) {
    println!("{}", kanji.kanji);

    println!("onyomi:");
    let mut t = term::stdout().unwrap();
    t.fg(term::color::BRIGHT_GREEN).unwrap();
    for onyomi in kanji.onyomis.iter() {
        writeln!(t, "{}", onyomi).unwrap();
    }
    t.reset().unwrap();
    
    println!("kunyomi:");
    t.fg(term::color::BRIGHT_BLUE).unwrap();
    for kunyomi in kanji.kunyomis.iter() {
        writeln!(t, "{}", kunyomi.0).unwrap();
    }

    t.reset().unwrap();
}

fn quiz_kanji(kanji: &quizlib::Kanji) {
    println!("{} onyomi: ", kanji.kanji);
    
    let onyomi_read: String = read!("{}\n");
    let onyomi_answers: Vec<String> = onyomi_read.trim().split_whitespace().map(|s| quizlib::romaji_to_katakana(s)).collect();
    println!("");

    let mut correct: Vec<String> = Vec::new();
    let mut incorrect: Vec<String> = Vec::new();
    let mut missed: Vec<String> = Vec::new();

    for onyomi in onyomi_answers.iter() {
        if kanji.onyomis.contains(&onyomi.to_string()) {
            correct.push(onyomi.to_string());
        } else {
            incorrect.push(onyomi.to_string());
        }
    }

    println!("{} kunyomi: ", kanji.kanji);
    
    let kunyomi_read: String = read!("{}\n");
    let kunyomi_answers: Vec<String> = kunyomi_read.trim().split_whitespace().map(|s| quizlib::romaji_to_hiragana(s)).collect();
    let kunyomi_strings: Vec<String> = kanji.kunyomis.iter().map(|s| s.0.clone()).collect();
    println!("");

    
    for kunyomi in kunyomi_answers.iter() {
        if kunyomi_strings.contains(&kunyomi.to_string()) {
            correct.push(kunyomi.to_string());
        } else {
            incorrect.push(kunyomi.to_string());
        }
    }

    for onyomi in kanji.onyomis.iter() {
        if !onyomi_answers.contains(&onyomi) {
            missed.push(onyomi.to_string());
        }
    }

    for kunyomi in kunyomi_strings.iter() {
        if !kunyomi_answers.contains(&kunyomi) {
            missed.push(kunyomi.to_string());
        }
    }

    let mut t = term::stdout().unwrap();

    if correct.len() > 0 {
        t.fg(term::color::BRIGHT_GREEN).unwrap();
        writeln!(t, "{}", correct.join(",")).unwrap();
    }

    if incorrect.len() > 0 {
        t.fg(term::color::BRIGHT_RED).unwrap();
        writeln!(t, "{}", incorrect.join(",")).unwrap();
    }

    if missed.len() > 0 {
        t.fg(term::color::BRIGHT_YELLOW).unwrap();
        writeln!(t, "{}", missed.join(",")).unwrap();
    }
    
    t.reset().unwrap();
}

/*
BLACK 	
BLUE 	
BRIGHT_BLACK 	
BRIGHT_BLUE 	
BRIGHT_CYAN 	
BRIGHT_GREEN 	
BRIGHT_MAGENTA 	
BRIGHT_RED 	
BRIGHT_WHITE 	
BRIGHT_YELLOW 	
CYAN 	
GREEN 	
MAGENTA 	
RED 	
WHITE 	
YELLOW
*/

#[derive(Clone,Debug)]
struct KanjiRow {
    id: i32,
    kanji: String,
    onyomi: Option<String>,
    kunyomi: Option<(String,Option<u32>)>
}



fn get_kanji_with_okurigana(conn: Connection) -> Vec<quizlib::Kanji> {
    let mut stmt = conn.prepare("SELECT kanji.id, kanji.kanji, onyomi.onyomi, kunyomi.kunyomi, kunyomi.okurigana_index FROM kanji LEFT OUTER JOIN onyomi ON kanji.id = onyomi.kanji_id LEFT OUTER JOIN kunyomi ON kanji.id = kunyomi.kanji_id WHERE kunyomi.okurigana_index IS NOT NULL;").unwrap();
    let kanji_rows : Vec<KanjiRow> = stmt.query_map(&[], |row| {
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
    }).unwrap().map(|x| x.unwrap()).collect();
    kanji_rows_to_kanjis(kanji_rows)
}


fn kanji_rows_to_kanjis(kanji_rows: Vec<KanjiRow>) -> Vec<quizlib::Kanji> {
    let mut kanjis : HashMap <String, quizlib::Kanji> = HashMap::new();
    for kanji_row in kanji_rows {
        match kanjis.get(&kanji_row.kanji).cloned() {
            Some(kanji) => {
                let mut kk = kanji.clone();
                match kanji_row.onyomi {
                    Some(onyomi) => if !kk.onyomis.contains(&onyomi) {
                        kk.onyomis.push(onyomi)
                    },
                    None => ()
                }
                match kanji_row.kunyomi {
                    Some(kunyomi) => if !kk.kunyomis.contains(&kunyomi) {
                        kk.kunyomis.push(kunyomi)
                    },
                    None => ()
                }
                kanjis.insert(kanji_row.kanji,kk);
            },
            None => {
                let mut kk = quizlib::Kanji { kanji: kanji_row.kanji.clone(), onyomis: Vec::new(), kunyomis: Vec::new() };
                match kanji_row.onyomi {
                    Some(onyomi) => if !kk.onyomis.contains(&onyomi) {
                        kk.onyomis.push(onyomi)
                    },
                    None => ()
                }
                match kanji_row.kunyomi {
                    Some(kunyomi) => if !kk.kunyomis.contains(&kunyomi) {
                        kk.kunyomis.push(kunyomi)
                    },
                    None => ()
                }
                kanjis.insert(kanji_row.kanji,kk);
            }
        }
    }
    kanjis.iter().map(|(_, kanji)| kanji.clone()).collect()
}
    
fn get_kanjis(conn: Connection) -> Vec<quizlib::Kanji> {
    let mut stmt = conn.prepare("SELECT kanji.id, kanji.kanji, onyomi.onyomi, kunyomi.kunyomi, kunyomi.okurigana_index FROM kanji LEFT OUTER JOIN onyomi ON kanji.id = onyomi.kanji_id LEFT OUTER JOIN kunyomi ON kanji.id = kunyomi.kanji_id;
").unwrap();
    //let mut kanjis : HashMap <String, quizlib::Kanji> = HashMap::new();
    let kanji_rows : Vec<KanjiRow> = stmt.query_map(&[], |row| {
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
    }).unwrap().map(|x| x.unwrap()).collect();
    kanji_rows_to_kanjis(kanji_rows)
        /*
    for kanji_row in kanji_rows {
        match kanjis.get(&kanji_row.kanji).cloned() {
            Some(kanji) => {
                let mut kk = kanji.clone();
                match kanji_row.onyomi {
                    Some(onyomi) => kk.onyomis.push(onyomi),
                    None => ()
                }
                match kanji_row.kunyomi {
                    Some(kunyomi) => kk.kunyomis.push(kunyomi),
                    None => ()
                }
                kanjis.insert(kanji_row.kanji,kk);
            },
            None => {
                let mut kk = quizlib::Kanji { kanji: kanji_row.kanji.clone(), onyomis: Vec::new(), kunyomis: Vec::new() };
                match kanji_row.onyomi {
                    Some(onyomi) => kk.onyomis.push(onyomi),
                    None => ()
                }
                match kanji_row.kunyomi {
                    Some(kunyomi) => kk.kunyomis.push(kunyomi),
                    None => ()
                }
                kanjis.insert(kanji_row.kanji,kk);
            }
        }
    }
    
    kanjis.iter().map(|(_, kanji)| kanji.clone()).collect()
*/
}
