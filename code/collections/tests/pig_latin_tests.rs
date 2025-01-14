use collections::question_two::pig_latin;

#[test]
fn test_words_starting_with_consonants() {
    assert_eq!(pig_latin(&"hello".to_string()), "ello-hay");
    assert_eq!(pig_latin(&"world".to_string()), "orld-way");
    assert_eq!(pig_latin(&"rust".to_string()), "ust-ray");
}

#[test]
fn test_words_starting_with_vowels() {
    assert_eq!(pig_latin(&"apple".to_string()), "apple-hay");
    assert_eq!(pig_latin(&"end".to_string()), "end-hay");
    assert_eq!(pig_latin(&"inside".to_string()), "inside-hay");
}

#[test]
fn test_multiple_words() {
    assert_eq!(pig_latin(&"hello world".to_string()), "ello-hay orld-way");
    assert_eq!(pig_latin(&"i am rust".to_string()), "i-hay am-hay ust-ray");
}

#[test]
fn test_empty_and_whitespace() {
    assert_eq!(pig_latin(&"".to_string()), "");
    assert_eq!(pig_latin(&"   ".to_string()), "");
}
