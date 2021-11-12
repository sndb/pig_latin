use std::error::Error;
use std::io;
use std::io::prelude::*;

fn trim_punctuation<'a>(s: &'a str) -> (&'a str, &'a str) {
    s.split_at(
        s.find(|c: char| c.is_ascii_punctuation())
            .unwrap_or(s.len()),
    )
}

fn convert_word(word: &str) -> String {
    let vowels = "aeiouAEIOU";

    if word.len() == 0 {
        return "".to_string();
    }

    let mut first = word.chars().next().unwrap();
    let mut rest: String = word.chars().skip(1).collect();

    if vowels.contains(first) {
        format!("{}-hay", word).to_string()
    } else {
        if first.is_uppercase() {
            first = first.to_ascii_lowercase();
            rest = rest
                .chars()
                .next()
                .unwrap()
                .to_uppercase()
                .chain(rest.chars().skip(1))
                .collect()
        }
        format!("{}-{}ay", rest, first).to_string()
    }
}

fn convert_word_with_punctuation(s: &str) -> String {
    let (word, punctuation) = trim_punctuation(s);
    let word = convert_word(word);
    format!("{}{}", word, punctuation).to_string()
}

fn convert_string(s: &str) -> String {
    s.split_whitespace()
        .map(convert_word_with_punctuation)
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    io::stdout().write_all(convert_string(&buf).as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn empty_input() {
        assert_eq!("", convert_word_with_punctuation(""));
    }

    #[test]
    fn single_vowel_uppercase() {
        assert_eq!("I-hay", convert_word_with_punctuation("I"));
    }

    #[test]
    fn single_vowel_uppercase_punct() {
        assert_eq!("I-hay?!", convert_word_with_punctuation("I?!"));
    }

    #[test]
    fn single_vowel_lowercase() {
        assert_eq!("a-hay", convert_word_with_punctuation("a"));
    }

    #[test]
    fn single_vowel_lowercase_punct() {
        assert_eq!("a-hay?!", convert_word_with_punctuation("a?!"));
    }

    #[test]
    #[should_panic]
    fn single_consonant_uppercase() {
        convert_word_with_punctuation("C");
    }
}
