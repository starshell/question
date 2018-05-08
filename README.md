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
question = "0.2.1"
```

Then add `extern crate question;` to your crate root and run `cargo build` or `cargo update && cargo build` for your project. Detailed documentation for releases can be found on [docs.rs](https://docs.rs/question/) and the bleeding edge docs for the latest GitLab repository version can be found on [GitLab pages](http://starshell.gitlab.io/question/question/).

### Example

```rust
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
```

Examples can also be ran directly:

```sh
$ cargo run --example yes_no_with_defaults
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/examples/yes_no_with_defaults`
Continue? (Y/n) why
Continue? (Y/n) y
Onward then!
```

See [examples](examples/) for more.

## Contributing

The project is mirrored to GitHub, but all development is done on GitLab. Please use the [GitLab issue tracker](https://gitlab.com/starshell/question/issues). Don't have a GitLab account? Just email `incoming+starshell/question@gitlab.com` and those emails automatically become issues (with the comments becoming the email conversation).

To contribute to Question, please see [CONTRIBUTING](CONTRIBUTING.md).

## License

Question is distributed under the terms of both the MIT license. If this does not suit your needs for some reason please feel free to contact me, or open an issue.

See [LICENSE](LICENSE).
