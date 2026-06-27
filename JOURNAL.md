# Journal

## Week 1 — Setup
- Workspace + first crate running.
- Create repo on Github.
- Next: Rust Book ch. 1–6, Rustlings intro.

## Week 2 — Rust Foundations
- Chapter 1 Quiz Scores (+Notes):
  - 1.1: 100%
  - 1.2: 100%
  - 1.3: 100%

- Chapter 1 Notes:
  - This chapter got me comfortable using cargo and understanding the basic file structure of a Rust project.

- Chapter 2 Notes:
  - I can create separate runnable binaries inside of existing crates and ```cargo run``` them separately: ```cargo run -p week02-foundations --bin guessing_game```
  - RUST always brings the Prelude into scope.
  - Variables are immutable by default.
  - References (&) are also immutable by default.
  - Use parse() to convert a value from one type to another one.
  - Use ```loop``` to create an infinite loop. ```break``` stops the loop and ```continue``` restarts the loop.
  - Use ```match``` rather than ```.expect``` with Result in order to catch errors.

- Chapter 3 Quiz Scores (+Notes):
  - 3.1: 
    - 100%
    - 50% Thought the compiler will error if the const declaration does not follow Rust's naming convention. Actually ```let``` variables can only be declared inside a function!
    - 100%
  - 3.2:
    - 100%
    - 100%

- Chapter 3 Notes:
  - Set up Rustlings to do the exercises in parallel.
  - Create variables.rs in bin folder.
  - ```const``` as the name suggests can not be changed after assigning them a value. Type declaration is also necessary.
  - Constants can only be set to constant expressions, not to a value that will get computed at runtime (in contrast to compile time).
  - Rust’s naming convention for constants is to use all uppercase with underscores between words.

  - Rust is statically typed. It must know the data types of all variables at compile time.
  - A scalar type represents a single value, e.g integers, floating-point numbers, Booleans, and characters.
  - Compound types can group multiple values into one type, e.g tuples and arrays.
  - Floats are always signed numbers.
  - Chars use single quotation marks.
  - Tuples can store values of multiple types. Fixed in length. ```let tup: (i32, f64, u8) = (500, 6.4, 1);```
  - Access single values of a tuple either by destructuring the tuple or by using .index notation.
  - An empty tuple is called a unit ```()```.
  - Arrays can store values of a specific type. Fixed in length. ```let a: [i32; 5] = [1, 2, 3, 4, 5];```
  - ```let a = [3, 3, 3, 3, 3];``` can also be written like this ```let a = [3; 5];```
  - Elements of an array x can be accessed like this ```x[0]```

  - Next up: Functions