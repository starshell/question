extern crate question;
use question::{Answer, Question};

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
