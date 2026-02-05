Level 1 – Basic TUI (Beginner)
==============================

Focus of this level
--------------------

In Level 1 you will:

- Understand raw mode and alternate screen mode in the terminal.
- Draw a simple single‑screen UI.
- Handle basic keyboard input (for example, using `q` to quit).

All exercises in this level should use the same basic building blocks:

- `crossterm` for terminal setup and input.
- `ratatui` for rendering blocks, borders, and text.

Suggested binary names for this level:

- `src/bin/ex01_hello_tui.rs`
- `src/bin/ex01_color_style.rs`


Exercise 1.1 – Hello TUI
------------------------

Goals
~~~~~

- Run your first TUI application.
- Understand the flow: initialize terminal, main loop, read input, and exit.

Description
~~~~~~~~~~~

- The application shows a single block with a simple title.
- Inside the block there is text with instructions to press `q` to quit.
- The app runs in full‑screen terminal mode and restores the terminal state when it exits.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex01_hello_tui.rs`

2. Implement a minimal TUI app that:

   - Enters raw mode and alternate screen mode.
   - Creates a `Terminal<CrosstermBackend<_>>`.
   - Enters a loop that:
     - Draws a block with a title such as "Hello TUI".
     - Shows a message like "Press q to quit".
   - Reads keyboard events and exits the loop when `q` is pressed.
   - Restores the terminal mode on exit (leaves alternate screen and disables raw mode).

3. Add personalization:

   - Replace the generic text inside the block with:
     - Your full name.
     - A short sentence about why you are learning Rust.

4. Improve exit keys:

   - Keep `q` to quit.
   - Add support for quitting with `Esc` as well.

Success criteria
~~~~~~~~~~~~~~~~

- The program compiles and runs without errors.
- The UI appears in a full‑screen terminal mode.
- Pressing `q` or `Esc` quits the application and restores the terminal state.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex01_hello_tui
```


Exercise 1.2 – Color and Style Experiments
------------------------------------------

Goals
~~~~~

- Get familiar with `Style`, `Color`, and `Modifier` in `ratatui`.
- Practice changing the visual appearance of text in the terminal.

Description
~~~~~~~~~~~

- Build a simple TUI that shows one main block with some text.
- The text and title should use different colors or styles to make the UI more readable.
- The user can toggle a "highlight" mode that changes the visual style.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex01_color_style.rs`

2. Start from the structure of Exercise 1.1:

   - Same basic loop: draw + read input.
   - Same terminal setup and teardown.

3. Change the visual style:

   - Use a non‑default foreground color for some text, for example:
     - `Color::Cyan`, `Color::Yellow`, or another color of your choice.
   - Apply one modifier such as:
     - `Modifier::BOLD`
     - `Modifier::ITALIC`

4. Implement two display modes:

   - **Normal mode**:
     - Use a basic style (for example, default color, no bold).
   - **Highlight mode**:
     - Use a noticeably different style (for example, bright color and bold).

5. Add input handling for mode switching:

   - Choose a key (for example `h`) to toggle between normal and highlight mode.
   - When highlight mode is active, the UI should clearly look different.

6. Keep the same quit behavior as Exercise 1.1:

   - `q` or `Esc` to quit.

Success criteria
~~~~~~~~~~~~~~~~

- Highlight mode changes the UI in a clear, visible way (color and/or style).
- Toggling highlight on and off works reliably.
- The code remains structured and easy to read.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex01_color_style
```

