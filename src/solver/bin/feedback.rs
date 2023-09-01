use crate::config::WORD_LENGTH;

#[derive(Debug)]
pub enum CharFeedback {
    NoMatch(char),
    WrongPosition(char, usize),
    ExactMatch(char, usize),
}

pub struct Feedback {
    pub chars_feedback: Vec<CharFeedback>,
    pub not_valid_word: bool,
}

impl Feedback {
    pub fn new() -> Self {
        let mut chars_feedback: Vec<CharFeedback> = vec![];
        chars_feedback.reserve(WORD_LENGTH);

        Self {
            chars_feedback,
            not_valid_word: false,
        }
    }
}
