# Journal

## Week 1 — Setup

- Workspace + first crate running.
- Created repo on GitHub.
- **Next:** Rust Book ch. 1–6, Rustlings intro.

---

## Week 2 — Rust Foundations

### Chapter 1 — Getting Started

**Quiz:** 1.1 · 1.2 · 1.3 — all 100%.

Got comfortable with `cargo` and the basic file structure of a Rust project.

### Chapter 2 — Guessing Game

- Separate runnable binaries can live inside an existing crate and run individually:
  `cargo run -p week02-foundations --bin guessing_game`
- The Prelude is always brought into scope automatically.
- Variables are immutable by default; references (`&`) are too.
- `parse()` converts a value from one type to another.
- `loop` makes an infinite loop — `break` stops it, `continue` restarts it.
- Prefer `match` over `.expect` with `Result` to handle errors.

### Chapter 3 — Common Programming Concepts

**Quiz**

| Section | Result | Note on misses |
|---|---|---|
| 3.1 | 100% / **50%** / 100% | Wrongly thought the compiler errors when a `const` ignores the naming convention. Real lesson: `let` can't be used at module/global scope — it only works inside a function or block, whereas `const` can be declared anywhere. |
| 3.2 | 100% / 100% | — |
| 3.3 | 100% | — |

**Setup**

- Set up Rustlings to run exercises in parallel.
- Created `variables.rs` in the `bin` folder.

**Constants**

- `const` can't change after assignment, and the type must be declared.
- Must be set to a constant expression — no runtime-computed values (compile-time only).
- Naming convention: all uppercase with underscores, e.g. `MAX_POINTS`.

**Types**

- Rust is statically typed: all variable types are known at compile time.
- *Scalar* types hold a single value: integers, floats, Booleans, chars.
- *Compound* types group values: tuples and arrays.
- Floats are always signed.
- Chars use single quotes.
- Tuples: mixed types, fixed length — `let tup: (i32, f64, u8) = (500, 6.4, 1);`. Access by destructuring or `.index`. An empty tuple `()` is the *unit*.
- Arrays: one type, fixed length — `let a: [i32; 5] = [1, 2, 3, 4, 5];`. Shorthand `[3; 5]` == `[3, 3, 3, 3, 3]`. Access with `a[0]`.

**Functions**

- Names use `snake_case`.
- Definition order doesn't matter, as long as they're visible to the caller.
- Parameter types must be declared in the signature.
- *Statements* perform an action and return no value; *expressions* evaluate to a value.