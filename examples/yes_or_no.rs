extern crate question;
use question::{Answer, Question};

fn main() {
    let answer = Question::new("Continue?").confirm();
    let correct = Answer::YES;
    assert_eq!(answer, correct);
}
