# My Rust Journey

### WTH is this repo
It's just my repo that documents my journey through learning the rust programming language using the [Rust Book](https://doc.rust-lang.org/stable/book/), I will follow the book and with some help of GPT 4o for the in depth explanation or if I dont understand te explanation in the book.

Im just a web developer that's really interested in rust and I want to discover a low level programming language and yeah, I had experience with C++ and some C#. I really liked how the rusteceans are like they are really friendly in the forums and they really inspire me to create and start learning and even start my own project using Rust :) 

### Resources
[The Rust Book](https://doc.rust-lang.org/stable/book/)

### My Space
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
As I have understand creates are like libraries and or packages that are created by other rustaceans 
