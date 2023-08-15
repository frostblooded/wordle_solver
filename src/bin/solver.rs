use std::{collections::HashMap, str::Chars};

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

fn pick_word(knowledge: &Knowledge) -> Option<&Word> {
    for word in &knowledge.available_words {
        if is_word_allowed(word, knowledge) {
            return Some(word);
        }
    }

    knowledge.available_words.first()
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
    let mut knowledge: Knowledge = Knowledge::new(words);

    loop {
        let selected_word: &Word = pick_word(&knowledge).expect("Couldn't pick word");
        println!("Selected word: {}", selected_word);

        let feedback: Vec<CharFeedback> = read_feedback(selected_word);
        knowledge.process_feedback(feedback);
        println!(
            "Known letters: {:?}\nMislaplced letters: {:?}\nExcluded letters: {:?}",
            knowledge.known_letters, knowledge.misplaced_letters, knowledge.excluded_letters
        );
    }
}
