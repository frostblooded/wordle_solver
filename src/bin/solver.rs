use std::{collections::HashMap, str::Chars};

const INPUT_FILE_PATH: &str = "resources/words_5_chars.txt";

type Word = String;
type Words = Vec<Word>;

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
}

impl Knowledge {
    fn new() -> Self {
        Self {
            known_letters: [None; 5],
            excluded_letters: vec![],
            misplaced_letters: HashMap::new(),
        }
    }

    fn process_feedback(&mut self, feedback: Vec<CharFeedback>) {
        for char_feedback in feedback {
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

fn pick_word(words: &Words) -> Option<&Word> {
    for word in words {
        let char_counts: HashMap<char, u8> = get_char_counts(word);

        if char_counts.values().any(|&v| v >= 2) {
            continue;
        }

        return Some(word);
    }

    words.first()
}

fn read_feedback(word: &Word) -> Vec<CharFeedback> {
    let mut raw_input: String = String::new();
    std::io::stdin()
        .read_line(&mut raw_input)
        .expect("Failed to read input");

    let mut chars: Chars = raw_input.trim().chars();
    let mut results: Vec<CharFeedback> = vec![];
    results.reserve(5);

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

        results.push(new_char_feedback)
    }

    results
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
    let mut knowledge: Knowledge = Knowledge::new();

    loop {
        let selected_word: &Word = pick_word(&words).expect("Couldn't pick word");
        println!("Selected word: {}", selected_word);

        let feedback: Vec<CharFeedback> = read_feedback(selected_word);
        knowledge.process_feedback(feedback);
        println!("Knowledge so far: {:?}", knowledge);
    }
}
