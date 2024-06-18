# 🦀 My Rusty Journey 🦀

## WTH is this repo

It's just my repo that documents my journey through learning the rust programming language using the [Rust Book](https://doc.rust-lang.org/stable/book/), I will follow the book and with some help of GPT 4o for the in depth explanation or if I dont understand te explanation in the book.

Im just a web developer that's really interested in rust and I want to discover a low level programming language and yeah, I had experience with C++ and some C#. I really liked how the rusteceans are like they are really friendly in the forums and they really inspire me to create and start learning and even start my own project using Rust :)

ps. Bruh i'm just a noob bare with me

## Resources

- [The Rust Book](https://doc.rust-lang.org/stable/book/)
- [Creates.io](https://crates.io/)

## Table of contents 📋

- [My Rusty Journey](#my-rusty-journey)
  - [WTH is this repo](#wth-is-this-repo)
  - [Resources](#resources)
  - [Table of contents 📋](#table-of-contents-)
  - [My Learnings](#my-learnings)
    - [1.Getting Started](#1getting-started)
      - [Cargo](#cargo)
      - [Creates](#creates)
    - [2. Programming a Guessing game](#2-programming-a-guessing-game)
      - [Setting up a project](#setting-up-a-project)
      - [Using IO, Vars, and print.](#using-io-vars-and-print)
      - [Using cargo to install crates](#using-cargo-to-install-crates)
      - [Random number generation](#random-number-generation)
      - [Ordering](#ordering)
      - [Looping](#looping)
      - [Error handling on parse](#error-handling-on-parse)

## My Learnings

### 1.Getting Started

Below will be the things I have learned so far and how I understand the language.
ps. I might use my native language sometimes

#### Cargo

Cargo is most likely the dependency manager of your project we can create a cargo setup by using the command below.
This command creates a new blank cargo project you can enter your project name in the `your_project_name` part

```bash
cargo new your_project_name
cd your_project_name
```

Cargo also has a few commands that helps the developer debug their program.
using the following commands like
This will compile the project then will run the binary under `./target/debug/your_project_name`

```bash
cargo run
```

This one will just create a binary inside the `./target/debug/your_project_name` path

```bash
cargo build
```

I think using build and run is just slow if you want to just check if your code is right. gladly they provided us with the command below

```bash
cargo check
```

cargo check is just designed to just check your code so that the compiler wont take time to create an executable when it's not needed.

#### Creates

As I have understand creates are like libraries and or packages that are created by other rustaceans. These are created packages to be easily used insead of creating one from scratch.

### 2. Programming a Guessing game

in this part of the book the book introduced me in a few stuff like;

- Functions
- Use statements
- IO
- Printing
- Using cargo to install crates
- Looping
- Error handling
- Ordering
- Rng

#### Setting up a project

Actually setting up the project is quite easy due to cargo being easy to use.

we just use the simple command below

```bash
cargo new guessing_game
```

This creates the init stuff of cargo where the src/main.rs is created and has a functioin that says hello world

```rust
fun main(){
    println!("Hello, World!")
}
```

#### Using IO, Vars, and print.

using the io is quite new to me the code below shows what is a mutable variable and how the program asks the user to input their answer

This also shows how the program shows the variables

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    //  Prompt the user for an input
    println!("Please input your guess.");

    // Create a blank mutable variable:string that is named guess
    let mut guess = String::new();
    /* the program reads the line or accept input and store it into the mut
      guess string. */
    io::stdin()
        /* & before the mut is to signal the compiler that we are passing
        a pointer of the guess variable rather passing the actual guess variable*/
        .read_line(&mut guess)
        //this part of the io is to handle potential failure with the results
        .expect("Failed to read line");
    //print the guess variable it will be placed in the {}
    println!("You guessed: {}", guess);
}
```

This part teaches me how to use mutable and imutable variables this reminds me of using const and let in javacript where const is immutable and let is mutable.

```rust
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

When using `println!("")` there are diffrent ways to display variables. Just like in js we can wrap the variable in a {} while in a quote so that it will show the variable's value.

```rust
let x = 5;
let y = 10;

// x = 5 and y + 2 = 12
println!("x = {x} and y + 2 = {}", y + 2);
```

#### Using cargo to install crates

It's not like NPM i think where i just type the dependency i need, when using cargo I need to put the dependency in the `[dependencies]` in the Cargo.toml file

Next is we're gonna create a new dependency in the `Cargo.toml` file

```toml
[dependencies]
rand = "0.8.5"
```

After we do put this dependency we just use the terminal and run this command to build and install the new dependencies

```bash
cargo build
```

#### Random number generation

After we run the cargo build i understand that those dependencies are used in the use statements it's like libraries that's called crates and they are founds in [creates.io](https://crates.io/) this is a registry of crates where you can find all the crates that's created by rustaceans.

We can now import the function that generates random numbers. This one generates from 1 to 100

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

#### Ordering

Now we Import another one called Ordering from the std library. This one helps us with conditions

Ordering is used when comparing two values using the `match` expression

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {guess}");

    // This one is also reffer the secret_number using the & sign
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

The book said:
First we add another `use` statement, bringing a type called `std::cmp::Ordering` into scope from the standard library. The `Ordering` type is another enum and has the variants `Less, Greater, and Equal`. These are the three outcomes that are possible when you compare two values.

Then we add five new lines at the bottom that use the `Ordering` type. The `cmp` method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing `guess` to `secret_number`. Then it returns a variant of the `Ordering` enum we brought into scope with the `use` statement. We use a `match` expression to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret_number`.

#### Looping

After having ordering we did the looping we just wraped the code we want to loop inside this

```rust
loop{
    //code you want to loop
    //if you want to stop the loop use
    break;
    //if you want to continue to the next iteration of the loop
    continue;
}
```

#### Error handling on parse

We also add a parsing error handling.

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

we changed the expect call to the match expression to crash the program on an error. this gives you more flexibility in handling the error so if the parse returns an ok it will pass the param num into num and put it into guess then if it's an error it will just continue and not parse.

so if i input `10` it will parse and put the value into guess then if i put `hello world` it wiill just continue the loop since ti's not a number and we cannot parse it

so all in all the code should look like this

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
   println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Enter your number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");

        //convert the guess string into a u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small bro"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("Nice! you winnn!!!");
                break;
            }
        }
    }
}
```
