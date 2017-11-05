extern crate question;
use question::{Question, Answer};

fn main() {
    let answer = Question::new("Continue?").confirm();
    let correct = Answer::YES;
    assert_eq!(answer, correct);
}
