// Convert strings to pig latin. The first consonant of each word
// is moved to the end of the word and _ay_ is added, so _first_
// becomes _irst-fay_. Words that start with a vowel have _hay_
// added to the end instead (_apple_ becomes _apple-hay_). Keep in
// mind the details about UTF-8 encoding!

pub fn pig_latin(input_string: &String) -> String {
    let mut pig_latin: String = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let words: Vec<&str> = input_string.trim().split_ascii_whitespace().collect();

    for (i, word) in words.iter().enumerate() {
        if let Some(first_char) = word.chars().nth(0) {
            if vowels.contains(&first_char) {
                pig_latin.push_str(&format!("{}-hay", word));
            } else {
                let rest = &word[1..];
                pig_latin.push_str(&format!("{}-{}ay", rest, first_char));
            }

            if i < words.len() - 1 {
                pig_latin.push(' ');
            }
        }
    }

    pig_latin
}
