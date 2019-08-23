Hi - welcome to Korny's "Rust 101" sample.

I'm not an expert, so I may well have missed some obvious improvements - feedback is welcome!

This is designed to work in one of two ways:

* by installing rust yourself, and using your editor-of-choice
* by using Visual Studio Code, and running the rust system inside Docker

The VSCode/Docker option is probably best if you just want to tinker, and don't want global tools on your system.

Installing rust yourself is faster, and gives you easier access to docs and things.  But it means yet another global app installed on your machine, probably in your path.  It's your choice.

## Using VSCode's container mode

Visual Studio Code has a quite neat development mode where you can develop code _inside a Docker container_

This means your machine only needs VSCode, and Docker - rust, and all it's tooling, and potentially rust-specific VSCode addons, all live in the container - you can keep the development totally isolated from your machine.  Handy if you just want to dip into this and not install rust globally.

See https://code.visualstudio.com/docs/remote/containers for more details and instructions.

In summary
* Make sure you have Docker installed
* Make sure you have Visual Studio Code installed
* Install the [Visual Studio Remote Development Extension](https://aka.ms/vscode-remote/download/extension)
* Open this project - you should get an "open in container" option - choose it!

_Note_ the first time you do this, VSCode will download a pile of Docker containers.  Be prepared to wait for a long time!

## Installing rust locally

See https://doc.rust-lang.org/book/ch01-01-installation.html - it's pretty straighforward, you'll get a `rustup` executable which installs the rust ecosystem for you.  `rustup update` updates you to the latest and greatest.

You then have `rustc` for compiling, or more usually you'll use `cargo` which is the rust package manager.

*Note* rust also installs _all of the rust documentation including the Rust book_ - run `rustup doc` to open the docs in a browser.

## Why did I build this?

Well, I wanted some things to "just work" - and to have a few opinionated defaults for a Rust 101 workshop:
- I wanted to use Rust 2018 edition (the default is still 2015)
- I wanted to encourage unit tests from the start
- I wanted a few useful packages:
  - logging
  - `failure` so you can use slightly easier generics
  - `pretty_assertions` as the default test assertions can be a mess to read

## Next steps

* Look at the book! https://doc.rust-lang.org/book/ or you can find it locally with `rustup doc`
* wait for me to write some more bits.
* feel free to start tinkering:

You can edit the main commandline entry point in `src/bin.rs` or the main library entry point in `src/lib.rs`

You can use `cargo test` to run tests, or `cargo run` to build and run the main method.

*Note* that [clippy](https://github.com/rust-lang/rust-clippy) is enabled by default - so lots of things might raise warnings, especially intermittent ones while you are typing.  Relax!  You can disable warnings if you need to, but clippy is awesome for suggesting better idioms and ways to do things.
