use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, Write},
};

use rand::seq::IteratorRandom;

use crate::{
    config::{INPUT_FILE_PATH, WORD_LENGTH},
    feedback::{CharFeedback, Feedback},
};

pub type Word = String;

#[derive(Debug)]
pub struct Knowledge {
    pub known_letters: [Option<char>; WORD_LENGTH],
    pub excluded_letters: HashSet<char>,
    pub misplaced_letters: HashMap<char, Vec<usize>>,
    pub available_words: Vec<Word>,
}

impl Knowledge {
    pub fn new(available_words: Vec<Word>) -> Self {
        Self {
            known_letters: [None; WORD_LENGTH],
            excluded_letters: HashSet::new(),
            misplaced_letters: HashMap::new(),
            available_words,
        }
    }

    fn process_no_match(&mut self, ch: char) {
        if self.known_letters.contains(&Some(ch)) {
            return;
        }

        if self.misplaced_letters.contains_key(&ch) {
            return;
        }

        self.excluded_letters.insert(ch);
    }

    fn process_wrong_position(&mut self, ch: char, pos: usize) {
        if let Some(ch_entry) = self.misplaced_letters.get_mut(&ch) {
            ch_entry.push(pos);
        } else {
            self.misplaced_letters.insert(ch, vec![pos]);
        }
    }

    fn process_exact_match(&mut self, ch: char, pos: usize) {
        let known_letter_cell: &mut Option<char> = self
            .known_letters
            .get_mut(pos)
            .unwrap_or_else(|| panic!("Known letters not initialized with {} chars?", WORD_LENGTH));

        *known_letter_cell = Some(ch);
    }

    pub fn process_feedback(&mut self, feedback: Feedback, word: &Word) {
        if feedback.not_valid_word {
            self.remove_available_word(word);
            self.update_available_words_input_file();
            return;
        }

        for char_feedback in &feedback.chars_feedback {
            if let CharFeedback::ExactMatch(ch, i) = char_feedback {
                self.process_exact_match(*ch, *i);
            }
        }

        for char_feedback in &feedback.chars_feedback {
            if let CharFeedback::WrongPosition(ch, i) = char_feedback {
                self.process_wrong_position(*ch, *i);
            }
        }

        for char_feedback in &feedback.chars_feedback {
            if let CharFeedback::NoMatch(ch) = char_feedback {
                self.process_no_match(*ch);
            }
        }
    }

    fn remove_available_word(&mut self, word: &Word) {
        self.available_words.retain(|w| w != word);
    }

    fn update_available_words_input_file(&self) {
        let output_file: File = File::create(INPUT_FILE_PATH)
            .expect("Failed to create and open input file for updating");
        let mut output_writer: io::BufWriter<File> = io::BufWriter::new(output_file);

        for word in &self.available_words {
            output_writer
                .write_fmt(format_args!("{}\n", word))
                .expect("Failed to write bytes to file");
        }
    }

    fn word_has_excluded_letters(&self, word: &Word) -> bool {
        for ch in word.chars() {
            if self.excluded_letters.contains(&ch) {
                return true;
            }
        }

        false
    }

    fn word_has_misplaced_letters(&self, word: &Word) -> bool {
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

    fn word_has_wrong_known_letters(&self, word: &Word) -> bool {
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

    fn is_word_allowed(&self, word: &Word) -> bool {
        !self.word_has_excluded_letters(word)
            && !self.word_has_misplaced_letters(word)
            && !self.word_has_wrong_known_letters(word)
    }

    pub fn pick_word(&self) -> Option<Word> {
        self.available_words
            .iter()
            .filter(|w| self.is_word_allowed(w))
            .choose(&mut rand::thread_rng())
            .cloned()
    }
}
