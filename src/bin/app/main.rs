//extern crate rusqlite;
#[macro_use] extern crate text_io;
extern crate quizlib;
extern crate term;

extern crate rand;

use rand::thread_rng;
use rand::Rng;

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

    let mkanjis = quizlib::get_kanji("pronunciation.txt");
    let kanjis = mkanjis.unwrap();
    //println!("{:?}", kanjis);
    //println!("{:?}", rand::thread_rng().choose(&kanjis.unwrap()));
    //     let num = rand::thread_rng().gen_range(0, 100);
    //    print_kanji((rand::thread_rng().choose(&kanjis)).unwrap());
    //    quiz_kanji((rand::thread_rng().choose(&kanjis)).unwrap());
    let ks = shuffle_copy(&kanjis);
    for k in ks.iter() {
        quiz_kanji(k);
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
        writeln!(t, "{}", kunyomi).unwrap();
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
