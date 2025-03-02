## Chapter 0: Cheat Sheet

**Goal:** Provide most of the default commands and settings used in the Rust projects.

## Information
**Search:** How hard is to learn Rust.\
**Prompt:** *rust compared to other languages*.\
**Result**: [Rust difficulty in comparison to other languages.](https://www.reddit.com/r/rust/comments/op2r7e/rust_difficulty_in_comparison_to_other_languages/?rdt=40201)

**Search:** How popular is the language.\
**Prompt:** *rust popularity*.\
**Result:** [Rust continues to be the most-admired programming language](https://www.reddit.com/r/rust/comments/1eb55ab/rust_continues_to_be_the_mostadmired_programming/).

**Search:** How paid is Rust.\
**Prompt:** *rust salary*.\
**Result:** [Rust Developer salary](https://web3.career/web3-salaries/rust-developer).

## Learning path
There are a lot of free resources to learn. Searching for most suitable for me, I came to the conclusion that the official documentation will be the best starting point. Also, I searched for some practical courses.I started with:
1. [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) book *by Steve Klabnik and Carol Nichols, with contributions from the Rust Community*.
2. [Learn Rust Programming - Complete Course](https://www.youtube.com/watch?v=BpPEoZW5IiY) by *Arfan Zubi*
3. [Learn to Code with Rust](https://softserve.udemy.com/course/learn-to-code-with-rust/) by *Boris Paskhaver*

## Prepare environment
First step is to make Rust available on the local machine - macOS.\
In a terminal enter the command below to install Rust.
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
![Image 0.1](../media/image_0.1.png)

After Rust installation check if the programming language was installed successfully by using command below.
```shell
rustc --version
```

Also, to make sure that the package manager was installed successfully use the command below.
```shell
cargo --version
```
![Image 0.2](../media/image_0.2.png)

For the journey I use VS Code with *rust-analyzer* extension.\
![Image 0.3](../media/image_0.3.png)

## Hello world
As a consecrated way of starting learn a programming language, first program written in that specific language is *Hello World* and here is no exception.\
The *Hello World* project will be stored in `chapter_00` directory. Use the commands below to create the working directory and to move to it.
```shell
mkdir chapter_00
cd Rustaceaner/chapter_00 
```

Create the first Rust file named `main.rs` paste the content from file with same name, and build it.
```
rustc main.rs 
```

After the file was built, execute it using the command below and the result will be displayed in the terminal.
```shell
./main 
```
![Image 0.4](../media/image_0.4.png)

## Hello cargo
For projects with higher complexity than *Hello World* will be used `Cargo` package manager.\
Initialize a new project using *cargo* package manager named `chapter_2` using the command below. The output will be a brand-new project directory that contains `src` directory where all code will be located and the default *Hello World* file is generated. Also, there is generated `Cargo.toml` file that contains project metadata and all necessary dependencies for current project.
```shell
cargo new chapter_2
```
![Image 0.5](../media/image_0.5.png)

As the default file `main.rs` is generated it can be built using a cargo command from below.
```shell
cargo build
```
After the building the project it can be run using the path to the project as per command below. By default, there is created working version that is fast compiled, but is not optimized, and it is stored in `target/debug` subdirectory.
```shell
./target/debug/chapter_2 
```
![Image 0.6](../media/image_0.6.png)

It can be run using cargo specific command - *cargo run*.
```shell
cargo run
```
![Image 0.7](../media/image_0.7.png)

In development process it is anti-productive to build and run the project at every line of code that was written. To bust productivity during development process there is a specific command shown below that just will check the code.
```shell
cargo check
```
![Image 0.8](../media/image_0.8.png)

If there is an error in the code as in example below, on line 2 are missing some elements, on check command the error will be caught.
![Image 0.9](../media/image_0.9.png)

After development phase, for production code there should be used another command to compile the code. This command optimize the code execution. It is slower in compiling, but faster in execution of the compiled code.
```shell
cargo run --release
```
![Image 0.10](../media/image_0.10.png)

When there are used a *crate* - Rust collection of source code, if it is necessary to update the version of the crates use the command below.\
*Binary crate* - executable project, as *project_2* that ws created.\
*Library crate* - code to be used in other projects, non-executable.
```shell
cargo update
```
The structure of the project after run for development and for production purpose is as per image below.\
![Image 0.11](../media/image_0.11.png)

## Guess number
Initialize the 3rd chapter for a new project.
```shell
cargo new chapter_3
```

Replace the content of the `main.rs` with the content from below. Check the code and run it - compile and execute.
```shell
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please enter your guess number: ");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```
![Image 0.12](../media/image_0.12.png)

Beside *errors* that can occur in the code, there are also *warnings*. If from code above delete the `expect` and the code will not handle possible errors, there will be a warning as in image below.\
![Image 0.13](../media/image_0.13.png)

To use *crates* that are not built-in, there is need to add them in the *Cargo.toml* under *[dependencies]* section as per example below. And on new build apply it will install all necessary dependencies and the outside crates are usable in the code.
```shell
[dependencies]
rand = "0.8.5"
```
![Image 3.3](../media/image_0.14.png)

As the crates are new to the user, to read the documentation there should be run a special command to generate the documentation and open o local.
```shell
cargo doc --open
```
![Image 3.4](../media/image_0.15.png)

Use the code from `chapter_3/src/main.rs` for the final version of the chapter and play with the second code created using Rust.
![Image 3.5](../media/image_0.16.png)