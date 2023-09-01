use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
};

use crate::{
    config::{INPUT_FILE_PATH, WORD_LENGTH},
    feedback::{CharFeedback, Feedback},
};

pub type Word = String;

#[derive(Debug)]
pub struct Knowledge {
    pub known_letters: [Option<char>; WORD_LENGTH],
    pub excluded_letters: Vec<char>,
    pub misplaced_letters: HashMap<char, Vec<usize>>,
    pub available_words: Vec<Word>,
}

impl Knowledge {
    pub fn new(available_words: Vec<Word>) -> Self {
        Self {
            known_letters: [None; WORD_LENGTH],
            excluded_letters: vec![],
            misplaced_letters: HashMap::new(),
            available_words,
        }
    }

    pub fn process_feedback(&mut self, feedback: Feedback, word: &Word) {
        if feedback.not_valid_word {
            self.remove_available_word(word);
            self.update_available_words_input_file();
            return;
        }

        for char_feedback in feedback.chars_feedback {
            match char_feedback {
                CharFeedback::NoMatch(ch) => self.excluded_letters.push(ch),
                CharFeedback::WrongPosition(ch, i) => {
                    if let Some(ch_entry) = self.misplaced_letters.get_mut(&ch) {
                        ch_entry.push(i);
                    } else {
                        self.misplaced_letters.insert(ch, vec![i]);
                    }
                }
                CharFeedback::ExactMatch(ch, i) => {
                    let known_letter_cell: &mut Option<char> =
                        self.known_letters.get_mut(i).unwrap_or_else(|| {
                            panic!("Known letters not initialized with {} chars?", WORD_LENGTH)
                        });
                    *known_letter_cell = Some(ch);
                }
            }
        }
    }

    pub fn remove_available_word(&mut self, word: &Word) {
        self.available_words.retain(|w| w != word);
    }

    pub fn update_available_words_input_file(&self) {
        let output_file: File = File::create(INPUT_FILE_PATH)
            .expect("Failed to create and open input file for updating");
        let mut output_writer: io::BufWriter<File> = io::BufWriter::new(output_file);

        for word in &self.available_words {
            output_writer
                .write_fmt(format_args!("{}\n", word))
                .expect("Failed to write bytes to file");
        }
    }

    pub fn word_has_excluded_letters(&self, word: &Word) -> bool {
        for ch in word.chars() {
            if self.excluded_letters.contains(&ch) {
                return true;
            }
        }

        false
    }

    pub fn word_has_misplaced_letters(&self, word: &Word) -> bool {
        for i in 0..WORD_LENGTH {
            let word_ch: char = word
                .chars()
                .nth(i)
                .unwrap_or_else(|| panic!("Word is shorter than {} chars?", WORD_LENGTH));

            if let Some(word_ch_misplaced_positions) = self.misplaced_letters.get(&word_ch) {
                if word_ch_misplaced_positions.contains(&i) {
                    return true;
                }
            }
        }

        for ch in self.misplaced_letters.keys() {
            if !word.contains(*ch) {
                return true;
            }
        }

        false
    }

    pub fn word_has_wrong_known_letters(&self, word: &Word) -> bool {
        for i in 0..WORD_LENGTH {
            let word_ch: char = word
                .chars()
                .nth(i)
                .unwrap_or_else(|| panic!("Word is shorter than {} chars?", WORD_LENGTH));

            if let Some(known_letter) = self
                .known_letters
                .get(i)
                .unwrap_or_else(|| panic!("Known letters array is not {} chars long?", WORD_LENGTH))
            {
                if word_ch != *known_letter {
                    return true;
                }
            }
        }

        false
    }
}
