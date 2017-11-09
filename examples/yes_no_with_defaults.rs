extern crate question;
use question::{Question, Answer};

fn main() {
    let answer = Question::new("Continue?")
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    if answer == Answer::YES {
        println!("Onward then!");
    } else {
        println!("Aborting...");
    }
}
