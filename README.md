# :crab: Rust Exercises from *The Rust Programming Language*

This repository contains solutions to various exercises I completed while reading *[The Rust Programming Language](https://doc.rust-lang.org/book/)* book. The exercises cover a wide range of Rust concepts, from basic syntax to advanced features. The purpose of this repository is to document my learning journey and provide solutions that may help others understand the key topics discussed in the book.

## Table of Contents

1. [:crab: Rust Exercises from *The Rust Programming Language*](#crab-rust-exercises-from-the-rust-programming-language)
   1. [Table of Contents](#table-of-contents)
   2. [Overview](#overview)
   3. [Prerequisites](#prerequisites)
   4. [Repository Structure](#repository-structure)
   5. [How to Use](#how-to-use)
   6. [Topics Covered](#topics-covered)
   7. [Contributing](#contributing)
      1. [How to Contribute](#how-to-contribute)
   8. [License](#license)

## Overview

*The Rust Programming Language*, often referred to as *The Book* in the Rust community, is a comprehensive guide to learning the Rust programming language. This repository contains my solutions and code samples for the exercises in the book. Each section focuses on different aspects of the language, allowing you to follow along with my progress and try the exercises yourself.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: Install the latest stable version of Rust from [rust-lang.org](https://www.rust-lang.org/tools/install)
- **Cargo**: The Rust package manager, which comes bundled with Rust.

You can verify your installation by running the following commands in your terminal:

```bash
rustc --version
cargo --version
```

## Repository Structure

The repository is organized by exercises from the book. Each chapter contains exercises, and each exercise is in its own directory.

```text
├── 01_hello_world/
│   └── main.rs
├── 01_hello_cargo/
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.lock
│   └── Cargo.toml
├── ...
├── LICENSE
└── README.md
```

- Each folder corresponds to an exercise.
- Each Rust source file (`.rs`) represents an exercise solution.
- Exercise naming pattern is `{chapter_number}_{exercise_shortname}/`

Feel free to browse the chapters and exercises to see how I approached the problems.

## How to Use

To run the exercises, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/ozefe/rust-exercises.git
    cd rust-exercises
    ```

2. Navigate to the exercise you want to run. For example:

    ```bash
    cd 01_hello_cargo
    ```

3. Compile and run the code using Cargo:

    ```bash
    cargo run
    ```

    Or if you’re working directly with `.rs` files:

    ```bash
    rustc main.rs
    ./main
    ```

Each exercise is self-contained and can be compiled and run individually.

## Topics Covered

The exercises in this repository touch upon a wide range of Rust topics, including but not limited to:

- Variables and Mutability
- Data Types
- Functions
- Control Flow (`if`, loops, etc.)
- Ownership, Borrowing, and Lifetimes
- `struct`s and `enum`s
- Pattern Matching
- Error Handling (`Result`, `Option`)
- Modules and Crates
- `trait`s and Generics
- Smart Pointers
- Concurrency

Each exercise is designed to reinforce the concepts explained in the corresponding chapter of *The Rust Programming Language*.

## Contributing

If you find any bugs in the solutions, or if you want to propose improvements, feel free to submit an issue or a pull request. Contributions are always welcome!

### How to Contribute

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Make your changes
4. Commit your changes (`git commit -m "Add feature"`)
5. Push to the branch (`git push origin feature-branch`)
6. Open a pull request

## License

This repository is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
