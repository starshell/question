use std::io::Write;
use std::collections::HashMap;

pub struct Question {
    question: String,
    prompt: String,
    default: Option<Answer>,
    acceptable: Option<Vec<String>>,
    valid_responses: Option<HashMap<String, Answer>>,
    clarification: Option<String>,
    tries: Option<u64>,
    until_acceptable: bool,
    show_defaults: bool,
    yes_no: bool,
}

impl Question {
    pub fn new(question: &str) -> Question {
        let question = question.to_string();
        Question {
            question: question.clone(),
            prompt: question,
            default: None,
            acceptable: None,
            valid_responses: None,
            clarification: None,
            tries: None,
            until_acceptable: false,
            show_defaults: false,
            yes_no: false,
        }
    }

    pub fn accept<'a>(&'a mut self, accepted: &str) -> &'a mut Question {
        let accepted = accepted.to_string();
        match self.acceptable {
            Some(ref mut vec) => vec.push(accepted),
            None => {
                let mut vec = Vec::new();
                vec.push(accepted);
                self.acceptable = Some(vec);
            },
        }
        self
    }

    pub fn acceptable<'a>(&'a mut self, accepted: &[String]) -> &'a mut Question {
        match self.acceptable {
            Some(ref mut vec) => vec.append(&mut accepted.to_vec()),
            None => {
                let vec = accepted.to_vec();
                self.acceptable = Some(vec);
            },
        }
        self
    }

    /// Shorhand for yes("yes") yes("y") no("no") no("n")
    pub fn yes_no<'a>(&'a mut self) -> &'a mut Question {
        self.yes_no = true;
        let response_keys = vec![
                                String::from("yes"),
                                String::from("y"),
                                String::from("no"),
                                String::from("n")
                            ];

        let response_values = vec![Answer::YES, Answer::YES, Answer::NO, Answer::NO];
        let mut valid_responses: HashMap<String, Answer> = response_keys.into_iter()
            .zip(response_values.into_iter())
            .collect();

        match self.valid_responses {
            Some(ref mut hashmap) => {
                for (k, v) in valid_responses.drain() {
                    hashmap.insert(k, v);
                }
            },
            None => self.valid_responses = Some(valid_responses),
        }
        self
    }

    pub fn tries<'a>(&'a mut self, tries: u64) -> &'a mut Question {
        match tries {
            0 => self.until_acceptable = true,
            1 => return self,
            _ => self.tries = Some(tries),
        }
        self
    }

    pub fn until_acceptable<'a>(&'a mut self) -> &'a mut Question {
        self.until_acceptable = true;
        self
    }

    pub fn show_defaults<'a>(&'a mut self) -> &'a mut Question {
        self.show_defaults = true;
        self
    }

    pub fn default<'a>(&'a mut self, answer: Answer) -> &'a mut Question {
        self.default = Some(answer);
        self
    }

    pub fn ask<'a>(&mut self) -> Option<Answer> {
        if self.yes_no {
            self.build_prompt();
        }
        let prompt = self.prompt.clone();
        self.printflush(prompt);

        let mut tries = 0;
        let valid_responses = self.valid_responses.clone().unwrap();
        loop {
            let input = self.user_input();
            for key in valid_responses.keys() {
                if *input.trim().to_lowercase() == *key {
                    return Some(valid_responses.get(key).unwrap().clone());
                }
            }
            if !self.until_acceptable {
                match self.tries {
                    None => return None,
                    Some(max_tries) if tries >= max_tries => return None,
                    Some(_) => tries += 1,
                }
            }
            self.build_clarification();
        }
    }

    /// Shorthand for yes_no() until_acceptable()
    pub fn confirm(&mut self) -> Answer {
        self.yes_no();
        self.build_prompt();
        let prompt = self.prompt.clone();
        self.printflush(prompt);

        let valid_responses = self.valid_responses.clone().unwrap();
        loop {
            let input = self.user_input();
            for key in valid_responses.keys() {
                if *input.trim().to_lowercase() == *key {
                    return valid_responses.get(key).unwrap().clone();
                }
            }
            self.build_clarification();
        }
    }

    #[cfg(not(test))]
    fn user_input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input
    }

    #[cfg(test)]
    fn user_input(&self) -> String {
        String::from("yes")
    }

    fn printflush<S>(&self, msg: S) where S: Into<String> {
        print!("{}", msg.into());
        std::io::stdout().flush().unwrap();
    }

    fn build_prompt(&mut self) {
        if self.show_defaults {
            match self.default {
                Some(Answer::YES) => self.prompt += "[Y/n]",
                Some(Answer::NO) => self.prompt += "[y/N]",
                None => self.prompt += "[y/n]",
                Some(_) => panic!(),
            }
        }
    }

    fn build_clarification(&mut self) {
        if let Some(clarification) = self.clarification.clone() {
            self.prompt = clarification;
            self.prompt += "\n";
            self.prompt += &self.question;
            self.build_prompt();
        }

    }
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Answer {
    RESPONSE(String),
    YES,
    NO,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_confirm() {
        let answer = Question::new("what is the meaning to life, the universe, and everything").confirm();
        assert_eq!(Answer::YES, answer);
    }
}
