//! An easy to use library for asking users questions when
//! designing Command Line Interface applications. Reduces
//! asking questions to a one liner.
//!
//! # Examples
//!
//! Asking a user a yes or no question requiring that
//! a valid response is provided.
//!
//! ```no_run
//! # use question::Question;
//! Question::new("Do you want to continue?").confirm();
//! ```
#![cfg_attr(feature = "strict", feature(plugin))]
#![cfg_attr(feature = "strict", plugin(clippy))]
#![cfg_attr(feature = "strict", deny(warnings))]

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

/// An `Answer` builder. Once a question has been formulated
/// either `ask` or `confirm` may be used to get an answer.
///
/// # Examples
///
/// The `ask` function will execute exactly the configuration
/// of the question. This will ask the user if they would like
/// to continue until they provide a valid yes or no response.
///
/// ```no_run
/// # use question::Question;
/// Question::new("Do you want to continue?")
///     .yes_no()
///     .until_acceptable()
///     .ask();
/// ```
///
/// The following `confirm` function is exactly equivalent.
///
/// ```no_run
/// # use question::Question;
/// Question::new("Do you want to continue?").confirm();
/// ```
#[derive(Clone)]
pub struct Question<R, W>
where
    R: Read,
    W: Write,
{
    question: String,
    prompt: String,
    default: Option<Answer>,
    clarification: Option<String>,
    acceptable: Option<Vec<String>>,
    valid_responses: Option<HashMap<String, Answer>>,
    tries: Option<u64>,
    until_acceptable: bool,
    show_defaults: bool,
    yes_no: bool,
    reader: R,
    writer: W,
}

impl Question<std::io::Stdin, std::io::Stdout> {
    /// Create a new `Question`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("What is your favorite color?").ask();
    /// ```
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
}

impl<R, W> Question<R, W>
where
    R: Read,
    W: Write,
{
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

    /// Add a single acceptable response to the list.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y" or "n" is entered.
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("Do you want to continue?")
    ///     .accept("y")
    ///     .accept("n")
    ///     .until_acceptable()
    ///     .ask();
    /// ```
    pub fn accept(&mut self, accepted: &str) -> &mut Question<R, W> {
        let accepted = accepted.to_string();
        match self.acceptable {
            Some(ref mut vec) => vec.push(accepted),
            None => {
                let mut vec = Vec::new();
                vec.push(accepted);
                self.acceptable = Some(vec);
            }
        }
        self
    }

    /// Add a collection of acceptable responses to the list.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y" or "n" is entered.
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("Do you want to continue?")
    ///     .acceptable(vec!["y", "n"])
    ///     .until_acceptable()
    ///     .ask();
    /// ```
    pub fn acceptable(&mut self, accepted: Vec<&str>) -> &mut Question<R, W> {
        let mut accepted = accepted.into_iter().map(|x| x.into()).collect();
        match self.acceptable {
            Some(ref mut vec) => vec.append(&mut accepted),
            None => self.acceptable = Some(accepted),
        }
        self
    }

    /// Shorthand the most common case of a yes/no question.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y", "n", "yes", or "no",
    /// is entered.
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("Do you want to continue?")
    ///     .yes_no()
    ///     .until_acceptable()
    ///     .ask();
    /// ```
    pub fn yes_no(&mut self) -> &mut Question<R, W> {
        self.yes_no = true;
        let response_keys = vec![
            String::from("yes"),
            String::from("y"),
            String::from("no"),
            String::from("n"),
        ];

        let response_values = vec![Answer::YES, Answer::YES, Answer::NO, Answer::NO];
        let mut valid_responses: HashMap<String, Answer> = response_keys
            .into_iter()
            .zip(response_values.into_iter())
            .collect();

        match self.valid_responses {
            Some(ref mut hashmap) => for (k, v) in valid_responses.drain() {
                hashmap.insert(k, v);
            },
            None => self.valid_responses = Some(valid_responses),
        }
        self
    }

    /// Set a maximum number of attempts to try and get an
    /// acceptable answer from the user.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y", "n", "yes", or "no",
    /// is entered, or until they have entered 3 invalid
    /// responses.
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("Do you want to continue?")
    ///     .yes_no()
    ///     .tries(3)
    ///     .ask();
    /// ```
    pub fn tries(&mut self, tries: u64) -> &mut Question<R, W> {
        match tries {
            0 => self.until_acceptable = true,
            1 => return self,
            _ => self.tries = Some(tries),
        }
        self
    }

    /// Never stop asking until the user provides an acceptable
    /// answer.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y", "n", "yes", or "no",
    /// is entered.
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("Do you want to continue?")
    ///     .yes_no()
    ///     .until_acceptable()
    ///     .ask();
    /// ```
    pub fn until_acceptable(&mut self) -> &mut Question<R, W> {
        self.until_acceptable = true;
        self
    }

    /// Show the default response to the user that will be
    /// submitted if they enter an empty string `"\n"`.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y", "n", "yes", or "no",
    /// is entered. Since no default was set the prompt will
    /// be displayed with `(y/n)` after it, and if the user
    /// enters an empty string no answer will be returned
    /// and they will be re-prompted.
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("Do you want to continue?")
    ///     .yes_no()
    ///     .until_acceptable()
    ///     .show_defaults()
    ///     .ask();
    /// ```
    ///
    /// If either `Answer::YES` or `Answer::NO` have been set
    /// as default then the prompt will be shown with that
    /// entry capitalized, either `(Y/n)` or `(y/N)`.
    pub fn show_defaults(&mut self) -> &mut Question<R, W> {
        self.show_defaults = true;
        self
    }

    /// Provide a default answer.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y", "n", "yes", "no", or
    /// "" an empty string is entered.  If an empty string
    /// is entered `Answer::YES` will be returned.
    ///
    /// ```no_run
    /// # use question::{Question, Answer};
    /// Question::new("Do you want to continue?")
    ///     .yes_no()
    ///     .until_acceptable()
    ///     .default(Answer::YES)
    ///     .show_defaults()
    ///     .ask();
    /// ```
    pub fn default(&mut self, answer: Answer) -> &mut Question<R, W> {
        self.default = Some(answer);
        self
    }

    /// Provide a clarification to be shown if the user does
    /// not enter an acceptable answer on the first try.
    ///
    /// # Examples
    ///
    /// The following will ask the user if they would like
    /// to continue until either "y", "n", "yes", "no", or
    /// "" an empty string is entered.  If an empty string
    /// is entered `Answer::YES` will be returned. If the
    /// user does not enter a valid response on the first
    /// attempt, the clarification will be added to the
    /// prompt.
    ///
    /// > Please enter either 'yes' or 'no'
    /// > Do you want to continue? (Y/n)
    ///
    /// ```no_run
    /// # use question::{Question, Answer};
    /// Question::new("Do you want to continue?")
    ///     .yes_no()
    ///     .until_acceptable()
    ///     .default(Answer::YES)
    ///     .show_defaults()
    ///     .clarification("Please enter either 'yes' or 'no'\n")
    ///     .ask();
    /// ```
    pub fn clarification(&mut self, c: &str) -> &mut Question<R, W> {
        self.clarification = Some(c.into());
        self
    }

    /// Ask the user a question exactly as it has been built.
    ///
    /// # Examples
    ///
    /// The following will return whatever the user enters
    /// as an `Answer::RESPONSE(String)`.
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("What is your favorite color?").ask();
    /// ```
    pub fn ask(&mut self) -> Option<Answer> {
        self.build_prompt();
        if self.until_acceptable {
            return Some(self.until_valid());
        }
        if self.tries.is_some() {
            return self.max_tries();
        }
        match self.get_response() {
            Ok(answer) => Some(answer),
            Err(_) => None,
        }
    }

    /// Ask a user a yes/no question until an acceptable
    /// response is given.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use question::Question;
    /// Question::new("Continue?").confirm();
    /// ```
    pub fn confirm(&mut self) -> Answer {
        self.yes_no();
        self.build_prompt();
        self.until_valid()
    }

    fn get_response(&mut self) -> Result<Answer, std::io::Error> {
        let prompt = self.prompt.clone();
        match self.prompt_user(&prompt) {
            Ok(ref answer) if (self.default != None) && answer == "" => {
                Ok(self.default.clone().unwrap())
            }
            Ok(answer) => Ok(Answer::RESPONSE(answer)),
            Err(e) => Err(e),
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
                    return Some(valid_responses[key].clone());
                }
                if let Some(default) = self.default.clone() {
                    if response == "" {
                        return Some(default);
                    }
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
                }
            }
        }
    }

    fn build_prompt(&mut self) {
        if self.show_defaults {
            match self.default {
                Some(Answer::YES) => self.prompt += " (Y/n)",
                Some(Answer::NO) => self.prompt += " (y/N)",
                Some(Answer::RESPONSE(ref s)) => {
                    self.prompt += " (";
                    self.prompt += s;
                    self.prompt += ")";
                }
                None => self.prompt += " (y/n)",
            }
        }
        self.prompt += " ";
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
        std::io::stdout().flush()?;
        let mut s = String::new();
        input.read_line(&mut s)?;
        Ok(String::from(s.trim()))
    }
}

/// An answer, the result of asking a `Question`.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Answer {
    /// A more complicated `RESPONSE(String)` that
    /// can be evaluated in the context of the
    /// application.
    RESPONSE(String),

    /// A "yes" answer.
    ///
    /// Used to represent any answers that are acceptable
    /// as a "yes" when asking a yes/no question.
    YES,

    /// A "no" answer.
    ///
    /// Used to represent any answers that are acceptable
    /// as a "no" when asking a yes/no question.
    NO,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn default_constructor() {
        let question = "Continue?";
        let q = Question::new(question);
        assert_eq!(question, q.question);
        assert_eq!(question, q.prompt);
        assert_eq!(None, q.default);
        assert_eq!(None, q.acceptable);
        assert_eq!(None, q.valid_responses);
        assert_eq!(None, q.clarification);
        assert_eq!(None, q.tries);
        assert_eq!(false, q.until_acceptable);
        assert_eq!(false, q.show_defaults);
        assert_eq!(false, q.yes_no);
    }


    #[test]
    fn set_default() {
        macro_rules! default {
            ( $question:expr, $set:expr, $expected:expr ) => {
                let mut q = Question::new($question);
                q.default($set);
                assert_eq!($expected, q.default.unwrap());
            }
        }
        let set = String::from("Yes Please!");
        let response = String::from("Yes Please!");
        default!("Continue?", Answer::NO, Answer::NO);
        default!("Continue?", Answer::YES, Answer::YES);
        default!(
            "Continue?",
            Answer::RESPONSE(set),
            Answer::RESPONSE(response)
        );
    }

    #[test]
    fn accept() {
        let mut q = Question::new("Continue?");

        q.accept("y");
        assert_eq!(vec!["y"], q.acceptable.unwrap());

        let mut q = Question::new("Continue?");
        q.accept("y");
        q.accept("yes");
        assert_eq!(vec!["y", "yes"], q.acceptable.unwrap());
    }

    #[test]
    fn acceptable() {
        let mut q = Question::new("Continue?");

        q.acceptable(vec!["y"]);
        assert_eq!(vec!["y"], q.acceptable.unwrap());

        let mut q = Question::new("Continue?");
        q.accept("y");
        q.acceptable(vec!["yes", "n", "no"]);
        assert_eq!(vec!["y", "yes", "n", "no"], q.acceptable.unwrap());
    }

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
        prompt!(
            "What is the only manmade object visable from the moon?",
            "The Great Wall of China"
        );
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

        ask!("y\n", "Continue?", "y");
        ask!("yes\n", "Continue?", "yes");
        ask!("n\n", "Continue?", "n");
        ask!("no\n", "Continue?", "no");
        ask!("the universe,\n", "42", "the universe,");
        ask!("and everything\n", "42", "and everything");
        ask!(
            "what is the meaning to life,\n",
            "42",
            "what is the meaning to life,"
        );

        ask!("y", "Continue?", "y");
        ask!("yes", "Continue?", "yes");
        ask!("n", "Continue?", "n");
        ask!("no", "Continue?", "no");
        ask!("the universe,", "42", "the universe,");
        ask!("and everything", "42", "and everything");
        ask!(
            "what is the meaning to life,",
            "42",
            "what is the meaning to life,"
        );
    }

    #[test]
    fn set_clarification() {
        macro_rules! confirm_clarification {
            ( $i:expr, $q:expr, $clarification:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let mut q = Question::with_cursor($q, input, output);
                q.clarification($clarification);
                assert_eq!($clarification, q.clarification.unwrap());
            }
        }
        confirm_clarification!("what is the meaning to life", "42", "14*3");
        confirm_clarification!("Continue?", "wat", "Please respond with yes/no");
    }

    #[test]
    fn set_max_tries() {
        macro_rules! confirm_max_tries {
            ( $i:expr, $q:expr, $max_tries:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let mut q = Question::with_cursor($q, input, output);
                q.tries($max_tries);
                assert_eq!($max_tries, q.tries.unwrap());
            }
        }
        confirm_max_tries!("what is the meaning to life", "42", 42);
        confirm_max_tries!("Continue?", "wat", 0x79);
    }

    #[test]
    fn set_until_acceptable() {
        macro_rules! confirm_until_acceptable {
            ( $i:expr, $q:expr, $until_acceptable:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let mut q = Question::with_cursor($q, input, output);
                q.until_acceptable();
                assert_eq!($until_acceptable, q.until_acceptable);
            }
        }
        confirm_until_acceptable!("what is the meaning to life", "42", true);
    }

    #[test]
    fn set_show_defaults() {
        macro_rules! confirm_show_defaults {
            ( $i:expr, $q:expr, $show_defaults:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let mut q = Question::with_cursor($q, input, output);
                q.show_defaults();
                assert_eq!($show_defaults, q.show_defaults);
            }
        }
        confirm_show_defaults!("what is the meaning to life", "42", true);
    }

    #[test]
    fn set_yes_no() {
        macro_rules! confirm_yes_no {
            ( $i:expr, $q:expr, $yes_no:expr ) => {
                let response = String::from($i);
                let input = Cursor::new(response.into_bytes());
                let output = Cursor::new(Vec::new());
                let mut q = Question::with_cursor($q, input, output);
                q.yes_no();
                assert_eq!($yes_no, q.yes_no);
            }
        }
        confirm_yes_no!("what is the meaning to life", "42", true);
    }
}
