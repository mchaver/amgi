//extern crate rusqlite;
#[macro_use] extern crate text_io;
extern crate quizlib;
extern crate term;

extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::io::BufRead;

use std::collections::HashMap;



fn shuffle_copy<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_vec();
    let mut rng = thread_rng();
    rng.shuffle(&mut newvec);
    newvec
}
//use rusqlite::Connection;
fn main() {
     let romaji_to_hiragana: HashMap<&str, &str> =
      [("a", "あ"),
       ("i", "い"),
       ("u", "う"),
       ("e", "え"),
       ("o", "お"),
       ("ka", "か"),
       ("ki", "き"),
       ("ku", "く"),
       ("ke", "け"),
       ("ko", "こ")]
       .iter().cloned().collect();
    /*
    let conn = Connection::open("test.sql").unwrap();
    let mut stmt = conn.prepare("SELECT kanji FROM kanji").unwrap();
    let rows = stmt.query_map(&[], |row| row.get(0)).unwrap();

    let mut names : Vec<String> = Vec::new();
    for name_result in rows {
        names.push(name_result.unwrap());
    }
    */
    /*
    for person in kanji_iter {
        println!("Found person {}", person);
    }
    */
    /*
    let mut stmt = conn.prepare("SELECT id, name, time_created, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            data: row.get(3)
        }
    }).unwrap();
    */
    println!("Hello from app");

    println!("{:?}", quizlib::is_hiragana('あ'));
    println!("{:?}", quizlib::is_hiragana('ア'));

    println!("{:?}", quizlib::hiragana_to_katakana('あ'));

    println!("{:?}", quizlib::to_katakana("ありがとうございます".to_string()));
    println!("{:?}", quizlib::to_hiragana("アリガトウゴザイマス".to_string()));


    println!("{:?}", quizlib::romaji_to_hiragana("kanada"));
    println!("{:?}", quizlib::romaji_to_hiragana("arigatou"));
    println!("{:?}", quizlib::romaji_to_hiragana("aiueo"));
    println!("{:?}", quizlib::romaji_to_hiragana("ryu"));
    println!("{:?}", quizlib::romaji_to_hiragana("annna"));
    println!("{:?}", quizlib::romaji_to_hiragana("kakka"));
    println!("{:?}", quizlib::romaji_to_hiragana("itteru"));
    
    println!("{:?}",  "hi" == "hi");
    println!("{:?}",  "hi" == "h");
    println!("{:?}",  "あ" < "か");
    println!("{:?}",  "a" < "ka");
    println!("{:?}",  "ka" < "dya");
    println!("{:?}",  "dya" < "ke");
    println!("{:?}",  "dya" < "e");

    let mut t = term::stdout().unwrap();

    t.fg(term::color::BRIGHT_GREEN).unwrap();
    write!(t, "hello, ").unwrap();

    t.fg(term::color::BRIGHT_BLUE).unwrap();
    writeln!(t, "world!").unwrap();

    t.reset().unwrap();

    let mkanjis = quizlib::get_kanji("pronunciation.txt");
    let kanjis = mkanjis.unwrap();
    //println!("{:?}", kanjis);
    //println!("{:?}", rand::thread_rng().choose(&kanjis.unwrap()));
    //     let num = rand::thread_rng().gen_range(0, 100);
//    print_kanji((rand::thread_rng().choose(&kanjis)).unwrap());

    let ks = shuffle_copy(&kanjis);
    for k in ks.iter() {
        quiz_kanji(k);
    }

//    quiz_kanji((rand::thread_rng().choose(&kanjis)).unwrap());
//    quiz_kanji((rand::thread_rng().choose(&kanjis)).unwrap());
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
        writeln!(t, "{}", kunyomi).unwrap();
    }

    t.reset().unwrap();
}

fn quiz_kanji(kanji: &quizlib::Kanji) {
    println!("{} onyomi: ", kanji.kanji);
    
    let onyomi_read: String = read!("{}\n");
    let onyomi_answers: Vec<String> = onyomi_read.trim().split_whitespace().map(|s| s.to_string()).collect();
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
    let kunyomi_answers: Vec<String> = kunyomi_read.trim().split_whitespace().map(|s| s.to_string()).collect();
    println!("");
    
    for kunyomi in kunyomi_answers.iter() {
        if kanji.kunyomis.contains(&kunyomi.to_string()) {
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

    for kunyomi in kanji.kunyomis.iter() {
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
