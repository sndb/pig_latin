use std::error::Error;
use std::io;
use std::io::prelude::*;

fn split_punctuation<'a>(s: &'a str) -> (&'a str, &'a str, &'a str) {
    let word = s.trim_matches(|c: char| c.is_ascii_punctuation());
    let i = s.find(word).unwrap();
    (&s[..i], &s[i..i + word.len()], &s[i + word.len()..])
}

fn is_vowel(c: char) -> bool {
    let vowels = "aeiouAEIOU";
    vowels.contains(c)
}

fn convert_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(c) if is_vowel(c) => {
            format!("{}-hay", word)
        }
        Some(c) => {
            if c.is_uppercase() {
                format!(
                    "{}-{}ay",
                    chars
                        .next()
                        .unwrap()
                        .to_uppercase()
                        .chain(chars)
                        .collect::<String>(),
                    c.to_ascii_lowercase()
                )
                .to_string()
            } else {
                format!("{}-{}ay", chars.collect::<String>(), c).to_string()
            }
        }
        None => "".to_string(),
    }
}

fn convert_word_with_punctuation(s: &str) -> String {
    match split_punctuation(s) {
        (prefix, word, suffix) => format!("{}{}{}", prefix, convert_word(word), suffix).to_string(),
    }
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
    fn single_vowel_uppercase_with_punctuation() {
        assert_eq!("I-hay?!", convert_word_with_punctuation("I?!"));
    }

    #[test]
    fn single_vowel_lowercase() {
        assert_eq!("a-hay", convert_word_with_punctuation("a"));
    }

    #[test]
    fn single_vowel_lowercase_with_punctuation() {
        assert_eq!("a-hay?!", convert_word_with_punctuation("a?!"));
    }

    #[test]
    fn word_with_both_sides_punctuation() {
        assert_eq!("!I-hay!", convert_word_with_punctuation("!Hi!"));
    }

    #[test]
    #[should_panic]
    fn single_consonant_uppercase() {
        convert_word_with_punctuation("C");
    }

    #[test]
    fn sentence() {
        assert_eq!("Ello-hay, orld-way!", convert_string("Hello, world!"));
    }
}
