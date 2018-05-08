extern crate question;
use question::{Answer, Question};

fn main() {
    let answer = Question::new("Continue?")
        .accept("y")
        .accept("n")
        .until_acceptable()
        .ask();
    let correct = Some(Answer::RESPONSE(String::from("y")));
    assert_eq!(answer, correct);
}
