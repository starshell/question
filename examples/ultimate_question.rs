extern crate question;
use question::{Answer, Question};

fn main() {
    let question = "What is the answer to the Ultimate Question of Life, \
                    the Universe, and Everything?";

    let answer = Question::new(question).ask().unwrap();
    let correct = Answer::RESPONSE(String::from("42"));
    assert_eq!(answer, correct);
}
