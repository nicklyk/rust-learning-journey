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
  - I can create separate runnable binaries inside of existing crates and ```carog run``` them separately: ```cargo run -p week02-foundations --bin guessing_game```
  - RUST always brings the Prelude into scope.
  - Variables are immutable by default.
  - Referrences (&) are also immutable by default.
  - Use parse() to convert a value from one type to another one.
  - Use ```loop``` to create an infinite loop. ```break``` stops the loop and ```continue``` restarts the loop.
  - Use ```match``` rather than ```.expect``` with Result in order to catch errors.