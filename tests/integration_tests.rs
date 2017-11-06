extern crate quizlib;

#[test]
fn romaji_to_hiragana() {
    assert_eq!("おいしい", quizlib::romaji_to_hiragana("oishii"));
    assert_eq!("かっこいい", quizlib::romaji_to_hiragana("kakkoii"));
    assert_eq!("かなだ", quizlib::romaji_to_hiragana("kanada"));
    assert_eq!("ありがとう", quizlib::romaji_to_hiragana("arigatou"));
    assert_eq!("あいうえお", quizlib::romaji_to_hiragana("aiueo"));
    assert_eq!("りゅ", quizlib::romaji_to_hiragana("ryu"));
    assert_eq!("かっかざん", quizlib::romaji_to_hiragana("kakkazann"));
    assert_eq!("いってる", quizlib::romaji_to_hiragana("itteru"));
    assert_eq!("あん", quizlib::romaji_to_hiragana("ann"));
    assert_eq!("あな", quizlib::romaji_to_hiragana("ana"));
    assert_eq!("あんな", quizlib::romaji_to_hiragana("annna"));
    assert_eq!("あんあ", quizlib::romaji_to_hiragana("anna"));

}

#[test]
fn romaji_to_katakana() {
    assert_eq!("オイシイ", quizlib::romaji_to_katakana("oishii"));
    assert_eq!("カッコイイ", quizlib::romaji_to_katakana("kakkoii"));
    assert_eq!("カナダ", quizlib::romaji_to_katakana("kanada"));
    assert_eq!("アリガトウ", quizlib::romaji_to_katakana("arigatou"));
    assert_eq!("アイウエオ", quizlib::romaji_to_katakana("aiueo"));
    assert_eq!("リュ", quizlib::romaji_to_katakana("ryu"));
    assert_eq!("カッカザン", quizlib::romaji_to_katakana("kakkazann"));
    assert_eq!("イッテル", quizlib::romaji_to_katakana("itteru"));
    assert_eq!("アン", quizlib::romaji_to_katakana("ann"));
    assert_eq!("アナ", quizlib::romaji_to_katakana("ana"));
    assert_eq!("アンナ", quizlib::romaji_to_katakana("annna"));
    assert_eq!("アンア", quizlib::romaji_to_katakana("anna"));

}


#[test]
fn hiragana_to_katakana() {
    assert_eq!("アリガトウゴザイマス", quizlib::to_katakana("ありがとうございます"));

}

#[test]
fn katakana_to_hiragana() {
    assert_eq!("ありがとうございます", quizlib::to_hiragana("アリガトウゴザイマス"));

}
