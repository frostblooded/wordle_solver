mod config;
mod feedback;
mod knowledge;

use config::INPUT_FILE_PATH;
use feedback::Feedback;
use knowledge::{Knowledge, Word};
use std::{iter::Peekable, str::Chars};

use crate::{config::WORD_LENGTH, feedback::CharFeedback};

fn pick_word(knowledge: &Knowledge) -> Option<Word> {
    for word in &knowledge.available_words {
        if knowledge.is_word_allowed(word) {
            return Some(word.clone());
        }
    }

    None
}

fn read_feedback(word: &Word) -> Feedback {
    println!("0 - letter not present in word\n1 - letter present but in wrong position\n2 - letter in exact position\nn - word is not recognized as a valid word");

    let mut raw_input: String = String::new();
    std::io::stdin()
        .read_line(&mut raw_input)
        .expect("Failed to read input");

    let mut chars: Peekable<Chars> = raw_input.trim().chars().peekable();
    let mut feedback: Feedback = Feedback::new();

    if chars.peek() == Some(&'n') {
        feedback.not_valid_word = true;
        feedback.chars_feedback.clear();
        return feedback;
    }

    let chars_feedback: &mut Vec<CharFeedback> = &mut feedback.chars_feedback;

    for i in 0..WORD_LENGTH {
        let word_ch: char = word
            .chars()
            .nth(i)
            .unwrap_or_else(|| panic!("Selected word was shorter than {} chars?!?", WORD_LENGTH));
        let feedback_ch: char = chars
            .next()
            .unwrap_or_else(|| panic!("Input feedback should be {} chars", WORD_LENGTH));

        let new_char_feedback: CharFeedback = match feedback_ch {
            '0' => CharFeedback::NoMatch(word_ch),
            '1' => CharFeedback::WrongPosition(word_ch, i),
            '2' => CharFeedback::ExactMatch(word_ch, i),
            _ => panic!("Wrong input, please input only 0, 1, 2!"),
        };

        chars_feedback.push(new_char_feedback)
    }

    feedback
}

fn read_input_words_file() -> Vec<String> {
    std::fs::read_to_string(INPUT_FILE_PATH)
        .expect("Failed to read input file")
        .split('\n')
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect()
}

fn main() {
    let words: Vec<String> = read_input_words_file();
    let mut knowledge: Knowledge = Knowledge::new(words);

    loop {
        let selected_word: Option<Word> = pick_word(&knowledge);

        if selected_word.is_none() {
            println!("Couldn't pick word. Exiting.");
            return;
        }

        let word: Word = selected_word.unwrap();
        println!("Selected word: {}", word);

        let feedback: Feedback = read_feedback(&word);
        knowledge.process_feedback(feedback, &word);
        println!(
            "Known letters: {:?}\nMislaplced letters: {:?}\nExcluded letters: {:?}",
            knowledge.known_letters, knowledge.misplaced_letters, knowledge.excluded_letters
        );
    }
}
