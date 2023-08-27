use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write},
    iter::Peekable,
    str::Chars,
};

const INPUT_FILE_PATH: &str = "resources/words_5_chars.txt";

type Word = String;

#[derive(Debug)]
enum CharFeedback {
    NoMatch(char),
    WrongPosition(char, usize),
    ExactMatch(char, usize),
}

#[derive(Debug)]
struct Knowledge {
    known_letters: [Option<char>; 5],
    excluded_letters: Vec<char>,
    misplaced_letters: HashMap<char, Vec<usize>>,
    available_words: Vec<Word>,
}

impl Knowledge {
    fn new(available_words: Vec<Word>) -> Self {
        Self {
            known_letters: [None; 5],
            excluded_letters: vec![],
            misplaced_letters: HashMap::new(),
            available_words,
        }
    }

    fn process_feedback(&mut self, feedback: Feedback, word: &Word) {
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
                    let known_letter_cell: &mut Option<char> = self
                        .known_letters
                        .get_mut(i)
                        .expect("Known letters not initialized with 5 chars?");
                    *known_letter_cell = Some(ch);
                }
            }
        }
    }

    fn remove_available_word(&mut self, word: &Word) {
        if let Ok(word_pos) = self.available_words.binary_search(word) {
            self.available_words.remove(word_pos);
        }
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
}

struct Feedback {
    chars_feedback: Vec<CharFeedback>,
    not_valid_word: bool,
}

impl Feedback {
    fn new() -> Self {
        let mut chars_feedback: Vec<CharFeedback> = vec![];
        chars_feedback.reserve(5);

        Self {
            chars_feedback,
            not_valid_word: false,
        }
    }
}

fn get_char_counts(word: &Word) -> HashMap<char, u8> {
    let mut results: HashMap<char, u8> = HashMap::new();

    for char in word.chars() {
        if let Some(char_entry) = results.get_mut(&char) {
            *char_entry += 1;
        } else {
            results.insert(char, 1);
        }
    }

    results
}

fn has_repeating_letters(word: &Word) -> bool {
    get_char_counts(word).values().any(|&v| v >= 2)
}

fn has_excluded_letters(word: &Word, knowledge: &Knowledge) -> bool {
    for ch in word.chars() {
        if knowledge.excluded_letters.contains(&ch) {
            return true;
        }
    }

    false
}

fn has_misplaced_letters(word: &Word, knowledge: &Knowledge) -> bool {
    for i in 0..5 {
        let word_ch: char = word.chars().nth(i).expect("Word is shorter than 5 chars?");

        if let Some(word_ch_misplaced_positions) = knowledge.misplaced_letters.get(&word_ch) {
            if word_ch_misplaced_positions.contains(&i) {
                return true;
            }
        }
    }

    false
}

fn has_wrong_known_letters(word: &Word, knowledge: &Knowledge) -> bool {
    for i in 0..5 {
        let word_ch: char = word.chars().nth(i).expect("Word is shorter than 5 chars?");

        if let Some(known_letter) = knowledge
            .known_letters
            .get(i)
            .expect("Known letters array is not 5 chars long?")
        {
            if word_ch != *known_letter {
                return true;
            }
        }
    }

    false
}

fn is_word_allowed(word: &Word, knowledge: &Knowledge) -> bool {
    !has_repeating_letters(word)
        && !has_excluded_letters(word, knowledge)
        && !has_misplaced_letters(word, knowledge)
        && !has_wrong_known_letters(word, knowledge)
}

fn pick_word(knowledge: &Knowledge) -> Option<Word> {
    for word in &knowledge.available_words {
        if is_word_allowed(word, knowledge) {
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

    for i in 0..5 {
        let word_ch: char = word
            .chars()
            .nth(i)
            .expect("Selected word was shorter than 5 chars?!?");
        let feedback_ch: char = chars.next().expect("Input feedback should be 5 chars");

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
