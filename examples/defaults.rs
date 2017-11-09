extern crate question;
use question::{Answer, Question};

fn main() {
    let question = "What is the answer to the Ultimate Question of Life, \
                    the Universe, and Everything?";

    let default = Answer::RESPONSE(String::from("42"));
    let answer = Question::new(question)
        .default(default.clone())
        .show_defaults()
        .ask()
        .unwrap();
    let correct = default;
    assert_eq!(answer, correct);
}
