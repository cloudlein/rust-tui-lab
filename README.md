Rust TUI Lab
============

This repository is a learning lab to practice building Text User Interfaces
(TUI) with Rust. It is structured to be:

- Clean and simple for beginners.
- Flexible enough to grow into a small TUI portfolio.
- Easy to navigate when you work through the exercises.


Learning Goals
--------------

- Understand the basics of TUI programming in the terminal using Rust.
- Learn how to use supporting crates, especially:
  - `crossterm` for working with the terminal, keyboard input, and terminal modes.
  - `ratatui` for building layouts and TUI widgets.
- Practice the event‑loop mindset: drawing the UI regularly and reacting to input.
- Learn to separate application state from rendering code.
- Build several small TUI applications as practice and portfolio material.


Prerequisites
-------------

- Rust and Cargo installed via `rustup`:
  - https://www.rust-lang.org/tools/install
- Operating systems: Windows, Linux, or macOS.
- For Windows with the MSVC toolchain:
  - Install Visual Studio 2017 or newer, or
  - Install "Build Tools for Visual Studio" with C++ (MSVC) and the Windows SDK.
- Basic Rust knowledge:
  - Variables, functions, `struct`, `enum`, `match`.
  - Basic use of `Result` and the `?` operator.


Project Structure
-----------------

- `Cargo.toml`
  - Package definition for `rust-tui-lab`.
  - Main dependencies:
    - `crossterm`
    - `ratatui`

- `src/`
  - You add your own `main.rs` and binaries as you work through the exercises.

- `src/bin/`
  - One `.rs` file per exercise (recommended).
  - Each file is compiled as a separate binary:
    - `cargo run --bin binary_name`

- `docs/`
  - `docs/exercises.md`: full exercise descriptions (all levels).


How to Run in General
---------------------

Make sure you are in the project directory:

```bash
cd path/to/rust-tui-lab
```

Basic commands:

- Check compilation:

```bash
cargo check
```

- Run an exercise binary:

```bash
cargo run --bin exercise_name
```

`exercise_name` is defined by how you name each binary for the corresponding
exercise. See the next section for suggested naming.


Recommended Project Layout While You Learn
-----------------------------------------

You are free to organize your files, but the following layout keeps things
simple and predictable:

- `src/main.rs` (optional)
  - Can be a simple launcher or left empty until you need it.

- `src/bin/`
  - One file per exercise:
    - `ex01_hello_tui.rs`        – Exercise 1.1
    - `ex01_color_style.rs`      – Exercise 1.2
    - `ex02_layout_demo.rs`      – Exercise 2.1
    - `ex02_dynamic_panel.rs`    – Exercise 2.2
    - `ex03_stateful_list.rs`    – Exercise 3.1
    - `ex03_list_action.rs`      – Exercise 3.2
    - `ex04_menu_pages.rs`       – Exercise 4.1
    - `ex04_input_prompt.rs`     – Exercise 4.2
    - `ex05_todo_app.rs`         – Exercise 5.1
    - `ex05_dashboard.rs`        – Exercise 5.2

Example commands:

```bash
cargo run --bin ex01_hello_tui
cargo run --bin ex02_layout_demo
```

You can adjust names if you prefer, but keep them consistent with the exercises
so that you always know which file belongs to which task.


Where to Find the Exercises
---------------------------

- High‑level overview and project structure:
  - This `README.md`.
- Full exercise descriptions (all levels):
  - `docs/exercises.md`

A good way to work is:

1. Open `docs/exercises.md`.
2. Pick the next exercise.
3. Create or update the corresponding file under `src/bin/`.
4. Run it with `cargo run --bin <exercise_binary_name>`.
5. Commit and push when you are done with that exercise.


Coding Guidelines (Summary)
---------------------------

- Prefer clear names over comments.
- Separate:
  - State (structs, enums).
  - Input handling.
  - Rendering.
- Use `match` for clean key handling.
- Handle edge cases for indices and `Option` values.


Portfolio Notes
---------------

If you use this repository as part of your GitHub portfolio:

- Keep commit messages focused on individual exercises or improvements.
- Consider adding a short checklist in `README.md` or the GitHub description,
  such as:
  - `[ ] Level 1`
  - `[ ] Level 2`
  - `[ ] Level 3`
  - `[ ] Level 4`
  - `[ ] Level 5`

This makes it clear to visitors how far you have progressed through the lab.

