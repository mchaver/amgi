use std::io::{BufReader,BufRead,Result};
use std::fs::File;
use std::path::Path;

use std::cmp::Ordering::{Equal,Greater,Less};

#[derive(Clone,Debug)]
pub struct Kanji {
    pub kanji: String,
    pub onyomis: Vec<String>,
    pub kunyomis: Vec<(String,Option<u32>)>
}

/*
pub fn get_kanji<P: AsRef<Path>>(file_name: P) -> Result<Vec<Kanji>> {
    let file = File::open(file_name).unwrap();
    let mut kanjis : Vec<Kanji> = vec![];
    for line in BufReader::new(file).lines() {
        let items =  line.unwrap();
        let v: Vec<&str> = items.split("|").collect();
        let onyomis: Vec<String> = v[1].split(",").map(|s| s.to_string()).filter(|s| s != "").collect();
        let kunyomis: Vec<String> = v[2].split(",").map(|s| s.to_string()).filter(|s| s != "").collect();        
        kanjis.push(Kanji{kanji: v[0].to_owned(), onyomis: onyomis.to_owned(), kunyomis: kunyomis.to_owned()});
    }
    Ok(kanjis)
}
*/
pub fn is_hiragana(c: char) -> bool {
    c >= '\u{3040}' && c <= '\u{309F}'
}

pub fn is_katakana(c: char) -> bool {
    c >= '\u{30A0}' && c <= '\u{30FF}'
}

// hiragana to katakana +60
// katakana to hiragana -60
// valid for conversion 3041 to 3096, 309D 309E
// 30A1 to 30A6, 30AD 30AE

pub const ROMAJI_TO_HIRAGANA_TABLE: &'static [(&str, &str)] = &[
    ("a","あ"),("ba","ば"),("be","べ"),("bi","び"),("bo","ぼ"),("bu","ぶ"),("bya","びゃ"),("byo","びょう"),("byu","びゅ"),("cha","ちゃ"),("chi","ち"),("cho","ちょ"),("chu","ちゅ"),("da","だ"),("de","で"),("di","ぢ"),("do","ど"),("du","づ"),("dya","ぢゃ"),("dyo","ぢょ"),("dyu","ぢゅ"),("e","え"),("fu","ふ"),("ga","が"),("ge","げ"),("gi","ぎ"),("go","ご"),("gu","ぐ"),("gya","ぎゃ"),("gyo","ぎょ"),("gyu","ぎゅ"),("ha","は"),("he","へ"),("hi","ひ"),("ho","ほ"),("hu","ふ"),("hya","ひゃ"),("hyo","ひょ"),("hyu","ひゅ"),("i","い"),("ja","じゃ"),("ji","じ"),("jo","じょ"),("ju","じゅ"),("ka","か"),("ke","け"),("ki","き"),("ko","こ"),("ku","く"),("kya","きゃ"),("kyo","きょ"),("kyu","きゅ"),("ma","ま"),("me","め"),("mi","み"),("mo","も"),("mu","む"),("mya","みゃ"),("myo","みょ"),("myu","みゅ"),("na","な"),("ne","ね"),("ni","に"),("nn","ん"),("no","の"),("nu","ぬ"),("nya","にゃ"),("nyo","にょ"),("nyu","にゅ"),("o","お"),("pa","ぱ"),("pe","ぺ"),("pi","ぴ"),("po","ぽ"),("pu","ぷ"),("pya","ぴゃ"),("pyo","ぴょお"),("pyu","ぴゅ"),("ra","ら"),("re","れ"),("ri","り"),("ro","ろ"),("ru","る"),("rya","りゃ"),("ryo","りょ"),("ryu","りゅ"),("sa","さ"),("se","せ"),("sha","しゃ"),("shi","し"),("sho","しょ"),("shu","しゅ"),("si","し"),("so","そ"),("su","す"),("ta","た"),("te","て"),("ti","ち"),("to","と"),("tsu","つ"),("tu","つ"),("u","う"),("wa","わ"),("wo","を"),("ya","や"),("yo","お"),("yu","う"),("za","ざ"),("ze","ぜ"),("zo","ぞ"),("zu","ず")
    /*
    uprint $ sort [("a","あ"), ("i","い"), ("u","う"), ("e","え"), ("o","お"),
    ("ka","か"), ("ki","き"), ("ku","く"), ("ke","け"), ("ko","こ"),
    ("sa","さ"), ("shi","し"), ("su","す"), ("se","せ"), ("so","そ"),
    ("ta","さ"), ("chi","ち"), ("tsu","つ"), ("te","て"), ("to","と"),
    ("na","な"), ("ni","に"), ("nu","ぬ"), ("ne","ね"), ("no","の"),
    ("ha","は"), ("hi","ひ"), ("fu","ふ"), ("he","へ"), ("ho","ほ"),
    ("ma","ま"), ("mi","み"), ("mu","む"), ("me","め"), ("mo","も"),
    ("ya","や"),              ("yu","う"),             ("yo","お"),
    ("ra","ら"), ("ri","り"), ("ru","る"), ("re","れ"), ("ro","ろ"),
    ("wa","わ"),                                       ("wo","を"),
                                                       ("nn","ん"),

    ("ga","が"), ("gi","ぎ"), ("gu","ぐ"), ("ge","げ"), ("go","ご"),
    ("za","ざ"), ("ji","じ"), ("zu","ず"), ("ze","ぜ"), ("zo","ぞ"),
    ("da","だ"), ("di","ぢ"), ("du","づ"), ("de","で"), ("do","ど"),
    ("ba","ば"), ("bi","び"), ("bu","ぶ"), ("be","べ"), ("bo","ぼ"),
    ("pa","ぱ"), ("pi","ぴ"), ("pu","ぷ"), ("pe","ぺ"), ("po","ぽ"),
    
    ("kya","きゃ"), ("kyu","きゅ"), ("kyo","きょ"),
    ("sha","しゃ"), ("shu","しゅ"), ("sho","しょ"),
    ("cha","ちゃ"), ("chu","ちゅ"), ("cho","ちょ"),
    ("nya","にゃ"), ("nyu","にゅ"), ("nyo","にょ"),
    ("hya","ひゃ"), ("hyu","ひゅ"), ("hyo","ひょ"),
    ("mya","みゃ"), ("myu","みゅ"), ("myo","みょ"), 
    ("rya","りゃ"), ("ryu","りゅ"), ("ryo","りょ"),

    ("gya","ぎゃ"), ("gyu","ぎゅ"), ("gyo","ぎょ"),
    ("ja","じゃ"), ("ju","じゅ"), ("jo","じょ"),
    ("dya","ぢゃ"), ("dyu","ぢゅ"), ("dyo","ぢょ"),
    ("bya","びゃ"), ("byu","びゅ"), ("byo","びょう"),
    ("pya","ぴゃ"), ("pyu","ぴゅ"), ("pyo","ぴょお"),

    ("si","し"), ("ti","ち"), ("tu","つ"), ("hu","ふ")]
        */
];

pub const KATAKANA_TO_HIRAGANA_TABLE: &'static [(char, char)] = &[
    ('\u{30A1}', '\u{3041}'), ('\u{30A2}', '\u{3042}'), ('\u{30A3}', '\u{3043}'), ('\u{30A4}', '\u{3044}'), ('\u{30A5}', '\u{3045}'), ('\u{30A6}', '\u{3046}'), ('\u{30A7}', '\u{3047}'), ('\u{30A8}', '\u{3048}'), ('\u{30A9}', '\u{3049}'), ('\u{30AA}', '\u{304A}'), ('\u{30AB}', '\u{304B}'), ('\u{30AC}', '\u{304C}'), ('\u{30AD}', '\u{304D}'), ('\u{30AE}', '\u{304E}'), ('\u{30AF}', '\u{304F}'), ('\u{30B0}', '\u{3050}'), ('\u{30B1}', '\u{3051}'), ('\u{30B2}', '\u{3052}'), ('\u{30B3}', '\u{3053}'), ('\u{30B4}', '\u{3054}'), ('\u{30B5}', '\u{3055}'), ('\u{30B6}', '\u{3056}'), ('\u{30B7}', '\u{3057}'), ('\u{30B8}', '\u{3058}'), ('\u{30B9}', '\u{3059}'), ('\u{30BA}', '\u{305A}'), ('\u{30BB}', '\u{305B}'), ('\u{30BC}', '\u{305C}'), ('\u{30BD}', '\u{305D}'), ('\u{30BE}', '\u{305E}'), ('\u{30BF}', '\u{305F}'), ('\u{30C0}', '\u{3060}'), ('\u{30C1}', '\u{3061}'), ('\u{30C2}', '\u{3062}'), ('\u{30C3}', '\u{3063}'), ('\u{30C4}', '\u{3064}'), ('\u{30C5}', '\u{3065}'), ('\u{30C6}', '\u{3066}'), ('\u{30C7}', '\u{3067}'), ('\u{30C8}', '\u{3068}'), ('\u{30C9}', '\u{3069}'), ('\u{30CA}', '\u{306A}'), ('\u{30CB}', '\u{306B}'), ('\u{30CC}', '\u{306C}'), ('\u{30CD}', '\u{306D}'), ('\u{30CE}', '\u{306E}'), ('\u{30CF}', '\u{306F}'), ('\u{30D0}', '\u{3070}'), ('\u{30D1}', '\u{3071}'), ('\u{30D2}', '\u{3072}'), ('\u{30D3}', '\u{3073}'), ('\u{30D4}', '\u{3074}'), ('\u{30D5}', '\u{3075}'), ('\u{30D6}', '\u{3076}'), ('\u{30D7}', '\u{3077}'), ('\u{30D8}', '\u{3078}'), ('\u{30D9}', '\u{3079}'), ('\u{30DA}', '\u{307A}'), ('\u{30DB}', '\u{307B}'), ('\u{30DC}', '\u{307C}'), ('\u{30DD}', '\u{307D}'), ('\u{30DE}', '\u{307E}'), ('\u{30DF}', '\u{307F}'), ('\u{30E0}', '\u{3080}'), ('\u{30E1}', '\u{3081}'), ('\u{30E2}', '\u{3082}'), ('\u{30E3}', '\u{3083}'), ('\u{30E4}', '\u{3084}'), ('\u{30E5}', '\u{3085}'), ('\u{30E6}', '\u{3086}'), ('\u{30E7}', '\u{3087}'), ('\u{30E8}', '\u{3088}'), ('\u{30E9}', '\u{3089}'), ('\u{30EA}', '\u{308A}'), ('\u{30EB}', '\u{308B}'), ('\u{30EC}', '\u{308C}'), ('\u{30ED}', '\u{308D}'), ('\u{30EE}', '\u{308E}'), ('\u{30EF}', '\u{308F}'), ('\u{30F0}', '\u{3090}'), ('\u{30F1}', '\u{3091}'), ('\u{30F2}', '\u{3092}'), ('\u{30F3}', '\u{3093}'), ('\u{30F4}', '\u{3094}'), ('\u{30F5}', '\u{3095}'), ('\u{30F6}', '\u{3096}'), ('\u{30FD}', '\u{309D}'), ('\u{30FE}', '\u{309E}')
];

pub const HIRAGANA_TO_KATAKANA_TABLE: &'static [(char, char)] = &[
    ('\u{3041}', '\u{30A1}'), ('\u{3042}', '\u{30A2}'), ('\u{3043}', '\u{30A3}'), ('\u{3044}', '\u{30A4}'), ('\u{3045}', '\u{30A5}'), ('\u{3046}', '\u{30A6}'), ('\u{3047}', '\u{30A7}'), ('\u{3048}', '\u{30A8}'), ('\u{3049}', '\u{30A9}'), ('\u{304A}', '\u{30AA}'), ('\u{304B}', '\u{30AB}'), ('\u{304C}', '\u{30AC}'), ('\u{304D}', '\u{30AD}'), ('\u{304E}', '\u{30AE}'), ('\u{304F}', '\u{30AF}'), ('\u{3050}', '\u{30B0}'), ('\u{3051}', '\u{30B1}'), ('\u{3052}', '\u{30B2}'), ('\u{3053}', '\u{30B3}'), ('\u{3054}', '\u{30B4}'), ('\u{3055}', '\u{30B5}'), ('\u{3056}', '\u{30B6}'), ('\u{3057}', '\u{30B7}'), ('\u{3058}', '\u{30B8}'), ('\u{3059}', '\u{30B9}'), ('\u{305A}', '\u{30BA}'), ('\u{305B}', '\u{30BB}'), ('\u{305C}', '\u{30BC}'), ('\u{305D}', '\u{30BD}'), ('\u{305E}', '\u{30BE}'), ('\u{305F}', '\u{30BF}'), ('\u{3060}', '\u{30C0}'), ('\u{3061}', '\u{30C1}'), ('\u{3062}', '\u{30C2}'), ('\u{3063}', '\u{30C3}'), ('\u{3064}', '\u{30C4}'), ('\u{3065}', '\u{30C5}'), ('\u{3066}', '\u{30C6}'), ('\u{3067}', '\u{30C7}'), ('\u{3068}', '\u{30C8}'), ('\u{3069}', '\u{30C9}'), ('\u{306A}', '\u{30CA}'), ('\u{306B}', '\u{30CB}'), ('\u{306C}', '\u{30CC}'), ('\u{306D}', '\u{30CD}'), ('\u{306E}', '\u{30CE}'), ('\u{306F}', '\u{30CF}'), ('\u{3070}', '\u{30D0}'), ('\u{3071}', '\u{30D1}'), ('\u{3072}', '\u{30D2}'), ('\u{3073}', '\u{30D3}'), ('\u{3074}', '\u{30D4}'), ('\u{3075}', '\u{30D5}'), ('\u{3076}', '\u{30D6}'), ('\u{3077}', '\u{30D7}'), ('\u{3078}', '\u{30D8}'), ('\u{3079}', '\u{30D9}'), ('\u{307A}', '\u{30DA}'), ('\u{307B}', '\u{30DB}'), ('\u{307C}', '\u{30DC}'), ('\u{307D}', '\u{30DD}'), ('\u{307E}', '\u{30DE}'), ('\u{307F}', '\u{30DF}'), ('\u{3080}', '\u{30E0}'), ('\u{3081}', '\u{30E1}'), ('\u{3082}', '\u{30E2}'), ('\u{3083}', '\u{30E3}'), ('\u{3084}', '\u{30E4}'), ('\u{3085}', '\u{30E5}'), ('\u{3086}', '\u{30E6}'), ('\u{3087}', '\u{30E7}'), ('\u{3088}', '\u{30E8}'), ('\u{3089}', '\u{30E9}'), ('\u{308A}', '\u{30EA}'), ('\u{308B}', '\u{30EB}'), ('\u{308C}', '\u{30EC}'), ('\u{308D}', '\u{30ED}'), ('\u{308E}', '\u{30EE}'), ('\u{308F}', '\u{30EF}'), ('\u{3090}', '\u{30F0}'), ('\u{3091}', '\u{30F1}'), ('\u{3092}', '\u{30F2}'), ('\u{3093}', '\u{30F3}'), ('\u{3094}', '\u{30F4}'), ('\u{3095}', '\u{30F5}'), ('\u{3096}', '\u{30F6}'), ('\u{309D}', '\u{30FD}'), ('\u{309E}', '\u{30FE}')
];

fn lookup_romaji_to_hiragana(s: &str, table: &'static [(&str,&str)]) -> Option<usize> {
    match table.binary_search_by(|&(key, _)| {
        if s == key { Equal }
        else if key < s { Less }
        else { Greater }
    }) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

// "b", "d", "f", "g", "h", "j", "k", "m", "p", "r", "s", "t", "v", "w", "z"
// x to shrink hirgana 
fn is_consonant(s: &str) -> bool {
    s.len() == 1 && !["a","e","i","n","o","u","y"].contains(&s)
}

pub fn romaji_to_hiragana(romaji: &str) -> String {
    let romaji_len = romaji.len();

    let mut romaji_index = 0;
    let mut window = 1; // 1 to 3
    let mut hiragana = "".to_string();

    // convert 1 to 3 ascii chars to a single hirgana
    // i is starting point, window is length to be looked up
    while romaji_index < romaji_len && romaji_index + window < romaji_len + 1 {
        let romaji_sub = &romaji[romaji_index .. romaji_index + window];

        // check for gemminate consonant, if it exists, consume the char
        // add a "っ" and skip the lookup
        if window == 1 && romaji_index + 1 < romaji_len && is_consonant(romaji_sub) {
            let romaji_peek_ahead = &romaji[romaji_index + 1 .. romaji_index + 2];

            if romaji_sub == romaji_peek_ahead {
                hiragana = format!("{}{}", hiragana, "っ");
                romaji_index += 1;
                window = 1;
                continue;
            }
        }
        
        match lookup_romaji_to_hiragana(romaji_sub, ROMAJI_TO_HIRAGANA_TABLE) {
            Some(hiragana_index) => {
                let single_hiragana = ROMAJI_TO_HIRAGANA_TABLE[hiragana_index].1.to_string();
                hiragana = format!("{}{}", hiragana, single_hiragana);
                romaji_index += window;
                window = 1;
            },
            None => {
                // romaji_sub was not found in the table, increase the window size if it was
                // not found. If the window is too large then move the index over and reset
                // the window size to 1
                if window < 3 {
                    window += 1;
                } else {
                    romaji_index += 1;
                    window = 1;
                }
            },
        }
    }
    
    hiragana.to_string()
}

pub fn romaji_to_katakana(romaji: &str) -> String {
    to_katakana(romaji_to_hiragana(romaji).as_str())
}

fn bsearch_case_table(c: char, table: &'static [(char, char)]) -> Option<usize> {
    match table.binary_search_by(|&(key, _)| {
        if c == key { Equal }
        else if key < c { Less }
        else { Greater }
    }) {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}

pub fn katakana_to_hiragana(c: char) -> char {
    match bsearch_case_table(c, KATAKANA_TO_HIRAGANA_TABLE) {
        None        => c,
        Some(index) => KATAKANA_TO_HIRAGANA_TABLE[index].1
    }
}

pub fn hiragana_to_katakana(c: char) -> char {
    match bsearch_case_table(c, HIRAGANA_TO_KATAKANA_TABLE) {
        None        => c,
        Some(index) => HIRAGANA_TO_KATAKANA_TABLE[index].1
    }
}

pub fn to_katakana(s: &str) -> String {
    s.chars()
     .collect::<Vec<char>>()
     .iter()
     .map(|c| hiragana_to_katakana(c.clone()))
     .into_iter()
     .collect()
}

pub fn to_hiragana(s: &str) -> String {
    s.chars()
     .collect::<Vec<char>>()
     .iter()
     .map(|c| katakana_to_hiragana(c.clone()))
     .into_iter()
     .collect()
}
