use std::io::Write;

pub struct Question {
    question: String,
}

impl Question {
    pub fn new(question: &str) -> Question {
        let question: String = question.to_string();
        Question { question }
    }

    pub fn confirm(self, default: Option<Answer>) -> Answer {
        let mut prompt: String = String::new();
        prompt += &self.question;
        match default {
            None => prompt += "[y/n]",
            Some(Answer::YES) => prompt += "[Y/n]",
            Some(Answer::NO) => prompt += "[y/N]",
            Some(_) => prompt += "[y/n]",
        };
        printflush(prompt);

        while let input = user_input() {
            if input.lenth() > 1 {
                continue;
            }
            
            input = input.strip().to_lowercase();

        }

        Answer::YES
    }
}

pub enum Answer {
    Some(String),
    YES,
    NO,
}

fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line")
}

fn printflush<S>(msg: S) where S: Into<String> {
    print!("{}", msg.into());
    std::io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
