use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

#[derive(Clone)]
pub struct Question<R, W>
    where R: Read,
          W: Write
{
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
    reader: R,
    writer: W,
}

impl<R, W> Question<R, W>
    where R: Read,
          W: Write,
{
    pub fn new(question: &str) -> Question<std::io::Stdin, std::io::Stdout> {
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
            reader: std::io::stdin(),
            writer: std::io::stdout(),
        }
    }

    #[cfg(test)]
    pub fn with_cursor(question: &str, input: R, output: W) -> Question<R, W> {
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
            reader: input,
            writer: output,
        }
    }

    pub fn accept<'f>(&'f mut self, accepted: &str) -> &'f mut Question<R, W> {
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

    pub fn acceptable<'f>(&'f mut self, accepted: &[String]) -> &'f mut Question<R, W> {
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
    pub fn yes_no<'f>(&'f mut self) -> &'f mut Question<R, W> {
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

    pub fn tries<'f>(&'f mut self, tries: u64) -> &'f mut Question<R, W> {
        match tries {
            0 => self.until_acceptable = true,
            1 => return self,
            _ => self.tries = Some(tries),
        }
        self
    }

    pub fn until_acceptable<'f>(&'f mut self) -> &'f mut Question<R, W> {
        self.until_acceptable = true;
        self
    }

    pub fn show_defaults<'f>(&'f mut self) -> &'f mut Question<R, W> {
        self.show_defaults = true;
        self
    }

    pub fn default<'f>(&'f mut self, answer: Answer) -> &'f mut Question<R, W> {
        self.default = Some(answer);
        self
    }

    pub fn clarification<'f>(&'f mut self, c: &str) -> &'f mut Question<R, W> {
        self.clarification = Some(c.into());
        self
    }

    pub fn ask(&mut self) -> Option<Answer> {
        if self.until_acceptable {
            return Some(self.until_valid());
        }
        if let Some(_) = self.tries {
            return self.max_tries();
        }
        match self.get_response() {
            Ok(answer) => Some(answer),
            Err(_) => None,
        }
    }

    /// Shorthand for yes_no() until_acceptable()
    pub fn confirm(&mut self) -> Answer {
        self.yes_no();
        self.build_prompt();
        self.until_valid()
    }

    fn get_response(&mut self) -> Result<Answer, std::io::Error> {
        let prompt = self.prompt.clone();
        match self.prompt_user(&prompt) {
            Ok(answer) => return Ok(Answer::RESPONSE(answer)),
            Err(e) => return Err(e),
        }
    }

    fn get_valid_response(&mut self) -> Option<Answer> {
        let prompt = self.prompt.clone();
        let valid_responses = match self.valid_responses.clone() {
            Some(thing) => thing,
            None => panic!(),
        };
        if let Ok(response) = self.prompt_user(&prompt) {
            for key in valid_responses.keys() {
                if *response.trim().to_lowercase() == *key {
                    return Some(valid_responses.get(key).unwrap().clone());
                }
            }
        }
        None
    }

    fn max_tries(&mut self) -> Option<Answer> {
        let mut attempts = 0;
        while attempts < self.tries.unwrap() {
            match self.get_valid_response() {
                Some(answer) => return Some(answer),
                None => {
                    self.build_clarification();
                    attempts += 1;
                    continue;
                }
            }
        }
        None
    }

    fn until_valid(&mut self) -> Answer {
        loop {
            match self.get_valid_response() {
                Some(answer) => return answer,
                None => {
                    self.build_clarification();
                    continue;
                },
            }
        }
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

    fn prompt_user(&mut self, question: &str) -> Result<String, std::io::Error> {
        let mut input = BufReader::new(&mut self.reader);
        write!(&mut self.writer, "{}", question)?;
        let mut s = String::new();
        input.read_line(&mut s)?;
        Ok(s)
    }

    #[cfg(test)]
    fn get_self(self) -> Question<R, W> {
        self
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
    use std::io::Cursor;

    #[test]
    fn prompt() {
        macro_rules! prompt {
            ( $question:expr, $user_input:expr ) => {
                let response = String::from($user_input);
                let input = Cursor::new(response.clone().into_bytes());
                let mut displayed_output = Cursor::new(Vec::new());
                let result;

                {
                    let mut q = Question::with_cursor($question, input, &mut displayed_output);
                    result = q.prompt_user($question).unwrap();
                } // end borrow of output before using it

                let output = String::from_utf8(displayed_output.into_inner()).expect("Not UTF-8");
                assert_eq!($question, output);
                assert_eq!(response, result);
            }
        }
        prompt!("what is the meaning to life", "42");
        prompt!("the universe", "42");
        prompt!("everything", "42");
        prompt!("Continue", "yes");
        prompt!("What is the only manmade object visable from the moon?", "The Great Wall of China");
    }

    #[test]
    fn basic_confirm() {
        macro_rules! confirm {
            ( $i:expr, $q:expr, $expected:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let actual = Question::with_cursor($q, input, output).confirm();
                assert_eq!($expected, actual);
            }
        }
        confirm!("y", "Continue?", Answer::YES);
        confirm!("yes", "Continue?", Answer::YES);
        confirm!("n", "Continue?", Answer::NO);
        confirm!("no", "Continue?", Answer::NO);
    }


    #[test]
    fn basic_ask() {
        macro_rules! ask {
            ( $i:expr, $q:expr, $expected:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let actual = Question::with_cursor($q, input, output).ask();
                assert_eq!(Some(Answer::RESPONSE(String::from($expected))), actual);
            }
        }
        ask!("y", "Continue?", "y");
        ask!("yes", "Continue?", "yes");
        ask!("n", "Continue?", "n");
        ask!("no", "Continue?", "no");
        ask!("what is the meaning to life,", "42", "what is the meaning to life,");
        ask!("the universe,", "42", "the universe,");
        ask!("and everything", "42", "and everything");
    }

    #[test]
    fn set_clarification() {
        macro_rules! confirm_clarification {
            ( $i:expr, $q:expr, $clarification:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let q = Question::with_cursor($q, input, output).clarification($clarification).get_self();
                assert_eq!($clarification, q.clarification.unwrap());
            }
        }
        confirm_clarification!("what is the meaning to life", "42", "14*3");
    }
}
