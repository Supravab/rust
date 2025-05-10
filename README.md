# rust

this is the important notes and documentation file for the entire journey of learning rust.
Only consist of main topics and important points, nothing more...

cargo check : this will check if the code is compilable without making it into executable
use case : periodically checking if the code is right without making it into an executable.

cargo init : initiallize the rust file using cargo, to include cargo into the project.

creating a new project is done by cargo new.

cargo stores the executable file in the target/debug

cargo build --release : this will create executable release file for cargo project, and put it in target/release

macro : it is basically a function that can take in variable number of arguements, it can check syntax before running.
example : println!("{}", variables); here the ! is the macro.

standard library documentation : https://doc.rust-lang.org/std/prelude/index.html
this library contains basic standard preludes(set of items that get into scope of programs)

in rust, by default variables are immutable(unchangable), to change the variable in future in the code,
you need to use mut while declaring the variable to declare it is changable.

String::new(), this will create a empty new string value.

like variables, references are immutable by default, therefore we need to mutate them to be able to use the reference.

learn about enum or enumeration..

crate : collection of rust source code files.
library crate : source code that is meant to be used in other programs and is not a single program itself.
eg : rand crate.

Cargo.toml : listed all the dependencies and crates. 8.5 means ^8.5 which will select everything higher than 8.5 less than 9.0

Cargo.lock : it will store the version of the dependencies used to build the project at first and use those versions to ensure that the package runs smoothly.
Cargo.lock doesnot change unless specified using Cargo.toml to change the version and rebuild the project.

We won't know which method, traits to use in a crate, so reading the documentation about the traits of a crate is necessary to use the crate properly.

cargo doc --open : locally builds a documentation html about the crate;

match : much like switch statement, it will compare the results, and give the output based on that.

enum values : parse() and read_line function of stdin give value with enum as well, basically Ok or Err, which can help us properly use the error management in each cases, if the program should continue, diverge or end.

enum is matched using match function, and the Ok(value) => value, will return the value if it is ok, true.
if Err(_) _means unknown, if the error is there no matter the error, it will do something
Err(_) => continue, this will ignore the error and continue
