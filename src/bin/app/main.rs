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
use std::io::stdin;
use std::collections::HashMap;

use termion::event::{Key, Event};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use std::io::{Read, Write};


fn shuffle_copy<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_vec();
    let mut rng = thread_rng();
    rng.shuffle(&mut newvec);
    newvec
}

#[derive(Debug)]
enum Mode {
    Learn = 1,
    Review,
    Quiz,
    Help,
}

fn input_to_mode(i: &str) -> Mode {
    match i {
        "1" => Mode::Learn,
        "2" => Mode::Review,
        "3" => Mode::Quiz,
        "4" => Mode::Help,
        _ => Mode::Help
    }
}

const MAIN_MENU: &'static str = "1: Learn
2: Review
3: Quiz
4: Help";

/*
quit \q
main menu \m
next \n
*/

//use rusqlite::Connection;
fn main() {
    let conn = Connection::open("test.sql").unwrap();
    let kanjis = get_kanjis(conn);
    print_kanji_chart(&kanjis);
    let ks = shuffle_copy(&kanjis);

    // read_string(&pprint);

    println!("{}", MAIN_MENU);
    let mode_read : String = read!("{}\n");
    let mode = input_to_mode(&mode_read.trim());
    println!();
    // println!("{:?}", mode);
    main_menu_select(mode, &ks);

}

fn pprint(s: &str) {
    println!("{}", s);
}

fn print_kanji_chart(kanjis: &Vec<quizlib::Kanji>) {
    let mut i = 0;
    for kanji in kanjis {
        i += 1;
        print!("{}", kanji.kanji);
        if i > 9 {
            println!();
            i = 0;
        }
    }
}


fn read_string(f: &Fn(&str)) {
    let input : String = read!("{}\n");
    f(&input);
}

fn main_menu_select(mode: Mode, kanjis: &Vec<quizlib::Kanji>) {
    match mode {
        Mode::Learn => for kanji in kanjis { learn_kanji(&kanji); },
        Mode::Review => review_kanjis(&kanjis),
        Mode::Quiz => for kanji in kanjis { quiz_kanji(&kanji); },
        Mode::Help => println!(""),
    }
}

// quiz: review mistakes


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

fn print_kunyomi(kunyomi: &(String,Option<u32>)) {
    let mut t = term::stdout().unwrap();

    match kunyomi.1 {
        Some(index) => {
            let (first,second) = index_split(&kunyomi.0, index);

            // write non-okurigana
            write!(t, "{}", first).unwrap();

            // write okurigana
            t.fg(term::color::GREEN).unwrap();
            write!(t, "{}\n", second).unwrap();

            // reset color
            t.reset().unwrap();
            io::stdout().flush();
        },
        None => write!(t, "{}\n", kunyomi.0).unwrap(),
    }
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
    println!("｢{}｣の音読み: {}\n", kanji.kanji, kanji.onyomis.join("、"));
    let mut skip_to_next = false;    

    for onyomi in kanji.onyomis.iter() {
        println!("{}", onyomi);
        let mut correct = 0;
        while correct < 3 && !skip_to_next {
            t.reset().unwrap();
            let onyomi_read_dirty: String = read!("{}\n");
            let onyomi_read = onyomi_read_dirty.trim();

            if onyomi_read == "\\q".to_string() {
                std::process::exit(0);
                
            } else if onyomi_read == "\\n".to_string() {
                skip_to_next = true;

            } else if onyomi_read == "\\s".to_string() {
                correct = 3;

            } else {
                let onyomis: Vec<String> = onyomi_read.split_whitespace().map(|s| s.to_string()).collect();
                // move up and delete previous line
                t.cursor_up();
                t.delete_line();

                for o in onyomis {
                    if quizlib::romaji_to_katakana(&o) == onyomi.to_string() {
                        t.fg(term::color::BRIGHT_GREEN).unwrap();
                        print!("{} ✓ ", o);
                        // increment correct counter
                        correct += 1;
                    } else {
                        t.fg(term::color::BRIGHT_RED).unwrap();
                        print!("{} × ", o);
                    }
                    t.reset().unwrap();
                    io::stdout().flush();
                }
                println!();
            }
        }

        println!();
    }

    print!("｢{}｣の訓読み: ", kanji.kanji);
    print_kunyomis(kanji.kunyomis.clone());
    println!("\n");
    
    for kunyomi in kanji.kunyomis.iter() {
        println!("{}", kunyomi.0);
        let mut correct = 0;
        while correct < 3 && !skip_to_next {
            t.reset().unwrap();
            let kunyomi_read_dirty: String = read!("{}\n");
            let kunyomi_read = kunyomi_read_dirty.trim();

            if kunyomi_read == "\\q".to_string() {
                std::process::exit(0);
                
            } else if kunyomi_read == "\\n".to_string() {
                skip_to_next = true;

            } else if kunyomi_read == "\\s".to_string() {
                correct = 3;

            } else {
                let kunyomis: Vec<String> = kunyomi_read.split_whitespace().map(|s| s.to_string()).collect();
                // move up and delete previous line
                t.cursor_up();
                t.delete_line();

                for k in kunyomis {
                    if quizlib::romaji_to_hiragana(&k) == kunyomi.0.to_string() {
                        t.fg(term::color::BRIGHT_GREEN).unwrap();
                        print!("{} ✓ ", k);
                        // increment correct counter
                        correct += 1;
                    } else {
                        t.fg(term::color::BRIGHT_RED).unwrap();
                        print!("{} × ", k);
                    }
                    t.reset().unwrap();
                    io::stdout().flush();
                }
                println!();
            }
        }

        println!();
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

// random, range
// x pronunciations less above equal
// repeat mistaken one

fn quiz_kanji(kanji: &quizlib::Kanji) {
    println!("{} 音読み: ", kanji.kanji);
    
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

    println!("{} 訓読み: ", kanji.kanji);
    
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
        writeln!(t, "正解：{}", correct.join(",")).unwrap();
    }

    if incorrect.len() > 0 {
        t.fg(term::color::BRIGHT_RED).unwrap();
        writeln!(t, "不正解：{}", incorrect.join(",")).unwrap();
    }

    if missed.len() > 0 {
        t.fg(term::color::BRIGHT_YELLOW).unwrap();
        writeln!(t, "未入力：{}", missed.join(",")).unwrap();
    }

    println!();
    
    t.reset().unwrap();

    if incorrect.len() > 0 || missed.len() > 0 {
        quiz_kanji(kanji);
    }
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
                let mut kk = quizlib::Kanji { id: kanji_row.id, kanji: kanji_row.kanji.clone(), onyomis: Vec::new(), kunyomis: Vec::new() };
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
    //v.sort_by(|a, b| a.cmp(b));
    //kanjis.sort_by(|a, b| a.id.cmp(b.id));
    let mut val : Vec<quizlib::Kanji> = kanjis.iter().map(|(_, kanji)| kanji.clone()).collect();
    val.sort_by(|a, b| a.id.cmp(&b.id));
    val
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
    let mut t = term::stdout().unwrap();
    
    for kanji in kanjis.iter() {
        println!("{}", kanji.kanji);
        let _pause : String = read!("{}\n");
        t.cursor_up();
        t.delete_line();
        
        for onyomi in kanji.onyomis.iter() {
            println!("{}", onyomi);
            let _inner_pause : String = read!("{}\n");
            t.cursor_up();
            t.delete_line();
        }

        for kunyomi in kanji.kunyomis.iter() {
            print_kunyomi(&kunyomi);
            let _inner_pause : String = read!("{}\n");
            t.cursor_up();
            t.delete_line();
        }

        println!("");
    }
}
