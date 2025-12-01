
#🦀 Match Control Flow in Rust

---

Understanding Pattern Matching with match, enum, and Option<T>

This project serves as a focused exploration of control flow using the match expression in Rust.
It expands on core concepts such as:

Enumerations with and without associated data

Pattern matching on enums

The special Option<T> enum

Exhaustive matching

Using _ as a catch-all pattern

Returning values from match arms

---

## 🎯 Learning Objectives

By completing this project, you will understand:

* How Rust’s match expression performs safe and exhaustive pattern matching

* How enums like Coin and Option<T> are used to encode different kinds of data

* Why match arms must return values of the same type

* How to destructure values inside enums (e.g., extracting UsState from Quarter)

* How _ and catch-all patterns behave in match expressions

* How match enables expressive, type-safe control flow

---

## ⚙️ Environment Setup

Please ensure that both Rust and Cargo are installed on your machine.

### 📁 Step 1 — Verify Installation
```bash
rustc --version
cargo --version
```

If Rust is not installed:
```bash
curl https://sh.rustup.rs -sSf | sh
```

After installing, check again using the commands above.

### 📁 Step 2 — Create or Navigate to the Project

To create a new Cargo project:
``` bash
cargo new match_control_flow
cd match_control_flow
```

Replace the contents of src/main.rs with the code below.
```bash
📜 Rust Code
#[derive(Debug)]       
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25,
    };
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // Catch-all pattern for all other values
    }
}
```

### ▶️ Step 3 — Build and Run the Project
Build
```bash
cargo build
```
Run
```bash
cargo run
```

---

## 🧠 Key Concepts Explained
Concept	Description
Enum with associated data	A variant like Quarter(UsState) stores additional information.
Pattern matching	match checks all possible patterns exhaustively.
Catch-all (_)	Matches everything not handled earlier. Useful for default behavior.
Option<T>	A built-in enum used for null-safe values (Some(T) or None).
Destructuring	Extracting data from inside enum variants (e.g., state from Quarter(state)).
Unit type ()	An empty tuple representing “no meaningful value.” Often used for placeholder actions.
## 🔍 How the Code Works
### 1️⃣ Matching Enum Variants
```bash
match coin {
    Coin::Penny => 1,
```

Each match arm returns a value, and all arms must return the same type.

Your commented examples returned integers and also attempted println!() (which returns ()), causing type mismatch errors — hence Rust’s strictness.

### 2️⃣ Enum with Data
```bash
Coin::Quarter(state)
```

This pattern binds the internal UsState value so it can be used inside the match arm if needed.

### 3️⃣ Matching Option<T>
```bash
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

This demonstrates why Option<T> is used everywhere — safe handling of potentially absent values.

### 4️⃣ Using _ Catch-All Pattern
```bash
_ => ()
```

This ensures the match expression is exhaustive while ignoring irrelevant cases.

### 5️⃣ Game Example with Match Control Flow

Shows how match can replace lengthy conditional chains with expressive patterns.

### 🧩 Example Output (Conceptual)

Since most functions contain no visible print output, running the program does not print values.
However, the logic demonstrates:

Using Option<i32> safely

Pattern matching on custom enums

Catch-all behavior in dice roll logic

---

## 🦀 Built With

Rust

Cargo

Pattern Matching

Enums & Option<T>

---

## 📄 License

This project is open-source and distributed under the MIT License.
