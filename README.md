# Question

[![Crates.io](https://img.shields.io/crates/v/question.svg)](https://crates.io/crates/question) [![Crates.io](https://img.shields.io/crates/d/question.svg)](https://crates.io/crates/question) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://gitlab.com/starshell/question/blob/master/LICENSE) [![Coverage Status](https://codecov.io/gl/starshell/question/branch/master/graph/badge.svg)](https://codecov.io/gl/starshell/question)

Linux: [![Build status](https://gitlab.com/starshell/question/badges/master/pipeline.svg)](https://gitlab.com/starshell/question/commits/master)
Windows: [![Build status](https://ci.appveyor.com/api/projects/status/k7ccce79080tfu18/branch/master?svg=true)](https://ci.appveyor.com/project/Eudoxier/question/branch/master)

A Command Line Question Asker for Rust.

> Ask a question, what more could you want?

Easy to use library for asking users questions when writing console/terminal applications.

## Usage

Add `question` as a dependency in your `Cargo.toml` to use from crates.io:

```toml
[dependencies]
question = "0.1.0"
```

Then add `extern crate question;` to your crate root and run `cargo build` or `cargo update && cargo build` for your project.

### Example

See [examples](examples/) for more.

```rust
extern crate question;
use question::{Question, Answer};

fn main() {
    let question = "What is the answer to the Ultimate Question of Life, the Universe, and Everything?";
    let answer = Question::new(question).ask().unwrap();
    let correct = Answer::RESPONSE(String::from("42"));
    assert_eq!(answer, correct);
}
```

Examples can also be ran directly:

```sh
$ cargo run --example yes_no_with_defaults
   Compiling question v0.1.0 (file:///home/datenstrom/workspace/starshell/question)
    Finished dev [unoptimized + debuginfo] target(s) in 8.75 secs
     Running `target/debug/examples/yes_no_with_defaults`
Continue? (Y/n) why
Continue? (Y/n) yes
```


## Contributing

To contribute to Question, please see [CONTRIBUTING](CONTRIBUTING.md).

## License

Question is distributed under the terms of both the MIT license. If this does not suit your needs for some reason please feel free to contact me, or open an issue.

See [LICENSE](LICENSE).
