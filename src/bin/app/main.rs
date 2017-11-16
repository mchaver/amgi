//extern crate rusqlite;
#[macro_use] extern crate text_io;
extern crate quizlib;
extern crate term;

extern crate rand;
extern crate rusqlite;
extern crate termion;

use rand::thread_rng;
use rand::Rng;
use rusqlite::Connection;
use std::io;

use std::collections::HashMap;

use termion::raw::IntoRawMode;
use std::io::Write;

fn shuffle_copy<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_vec();
    let mut rng = thread_rng();
    rng.shuffle(&mut newvec);
    newvec
}
//use rusqlite::Connection;
fn main() {
    let conn = Connection::open("test.sql").unwrap();
    let kanjis = get_kanjis(conn);
    let ks = shuffle_copy(&kanjis);    
    for kanji in ks {
        learn_kanji(&kanji);
    }
}

fn index_split(s: &String, i: u32) -> (String,String) {
    let mut first = "".to_string();
    let mut second = "".to_string();
    let mut ix = 0;
    for c in s.chars().collect::<Vec<char>>().iter() {
        if i > ix {
            first.push(c.clone());
        } else {
            second.push(c.clone());
        }
        ix += 1;
    }
    (first,second)
}

fn utf8_split(s: &String, i: u32) {
    let mut t = term::stdout().unwrap();
    let mut ix = 0;
    for c in s.chars().collect::<Vec<char>>().iter() {
        if i == ix {
            t.fg(term::color::GREEN).unwrap();
        }
        write!(t, "{}", c).unwrap();
        ix += 1;
    }
    t.reset().unwrap();
    io::stdout().flush();
}

fn print_kunyomis(kunyomis: Vec<(String,Option<u32>)>) {
    let mut t = term::stdout().unwrap();

    for (i, kunyomi) in kunyomis.iter().enumerate() {
        match kunyomi.1 {
            Some(index) => {
                let (first,second) = index_split(&kunyomi.0, index);

                // write non-okurigana
                write!(t, "{}", first).unwrap();

                // write okurigana
                t.fg(term::color::GREEN).unwrap();
                write!(t, "{}", second).unwrap();

                // reset color
                t.reset().unwrap();
                io::stdout().flush();
            },
            None => write!(t, "{}", kunyomi.0).unwrap(),
        }
        
        // add comma if not last
        if i < kunyomis.len() - 1 {
            write!(t, ",");
        }
    }
}

fn learn_kanji(kanji: &quizlib::Kanji) {
    let mut t = term::stdout().unwrap();
    println!("｢{}｣の音読み: {}", kanji.kanji, kanji.onyomis.join("、"));
    
    for onyomi in kanji.onyomis.iter() {
        println!("{}", onyomi);
        let mut correct = 0;
        while correct < 3 {
            t.reset().unwrap();
            let onyomi_read: String = read!("{}\n");
            if quizlib::romaji_to_katakana(onyomi_read.trim()) == onyomi.to_string() {
                // move up and delete previous line
                t.cursor_up();
                t.delete_line();
                // set to green
                t.fg(term::color::BRIGHT_GREEN).unwrap();
                println!("{} ✓", onyomi_read.trim());
                // reset
                t.reset().unwrap();
                io::stdout().flush();
                // increment correct counter
                correct += 1;
            } else {
                // move up and delete line
                t.cursor_up();
                t.delete_line();
                // set to red
                t.fg(term::color::BRIGHT_RED).unwrap();
                println!("{} ×", onyomi_read.trim());
                // reset
                t.reset().unwrap();
                io::stdout().flush();
            }
        }
    }

    //let s = kanji.kunyomis.iter().map(|o| o.0.clone()).collect::<Vec<String>>().join("、");
    //println!("｢{}｣の訓読み: {}", kanji.kanji, s);
    print!("｢{}｣の訓読み: ", kanji.kanji);
    print_kunyomis(kanji.kunyomis.clone());
    println!("");
    // println!("｢{}｣の訓読み: {}", kanji.kanji, kanji.kunyomis.iter().map(|o: (String,Option<u32>)| o.0).collect::<Vec<String>>().join("、"));
    
    for kunyomi in kanji.kunyomis.iter() {
        println!("{}", kunyomi.0);
        let mut correct = 0;
        while correct < 3 {
            t.reset().unwrap();
            let kunyomi_read: String = read!("{}\n");
            if quizlib::romaji_to_hiragana(kunyomi_read.trim()) == kunyomi.0.to_string() {
                // move up and delete previous line
                t.cursor_up();
                t.delete_line();
                // set to green
                t.fg(term::color::BRIGHT_GREEN).unwrap();
                println!("{} ✓", kunyomi_read.trim());
                // reset
                t.reset().unwrap();
                io::stdout().flush();
                // increment correct counter
                correct += 1;
            } else {
                // move up and delete line
                t.cursor_up();
                t.delete_line();
                // set to red
                t.fg(term::color::BRIGHT_RED).unwrap();
                println!("{} ×", kunyomi_read.trim());
                // reset
                t.reset().unwrap();
                io::stdout().flush();
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

// show kanji, reveal one pronunciation with each return
fn review_kanjis(kanjis: &Vec<quizlib::Kanji>) {
    for kanji in kanjis.iter() {
        
    }
}
