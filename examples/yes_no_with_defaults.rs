extern crate question;
use question::{Question, Answer};

fn main() {
    let answer = Question::new("Continue?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();
    let correct = Answer::YES;
    assert_eq!(answer, correct);
}
