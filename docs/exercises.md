Rust TUI Lab – Exercises
========================

This document contains the full sequence of exercises for the Rust TUI Lab,
from beginner to near‑expert level. Work through them in order unless you
have strong reasons to skip ahead.


Level 1 – Basic TUI (Beginner)
------------------------------

Focus of this level:

- Understanding raw mode and alternate screen mode in the terminal.
- Drawing a simple single‑screen UI.
- Handling basic keyboard input (for example, using `q` to quit).


Exercise 1.1 – Hello TUI
~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Run your first TUI application.
- Understand the flow: initialize terminal, main loop, read input, and exit.

Description:

- The application shows a single block with a simple title.
- Inside the block there is text with instructions to press `q` to quit.

Success criteria:

- The program compiles and runs without errors.
- The UI appears in a full‑screen terminal mode.
- Pressing `q` quits the application and restores the terminal state.

Tasks:

1. Change the text inside the block to:
   - Your full name.
   - A short sentence about why you are learning Rust.
2. Adjust the quit key:
   - In addition to `q`, also support quitting with `Esc`.
3. Add one more line of text under the instructions, for example:
   - "Try changing this code and run it again."

Example command:

```bash
cargo run --bin hello_tui
```


Exercise 1.2 – Color and Style Experiments
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Get familiar with `Style`, `Color`, and `Modifier` in `ratatui`.

Tasks:

1. Build a simple single‑block UI.
2. Use text with:
   - A non‑default foreground color (for example `Color::Cyan` or `Color::Yellow`).
   - One modifier such as `Modifier::BOLD` or `Modifier::ITALIC`.
3. Implement two display modes:
   - Normal mode.
   - "Highlight" mode.
4. Use a key (for example `h`) to toggle highlight mode on and off.

Success criteria:

- When highlight mode is active, the UI is clearly different (color or style).
- The code remains structured and easy to read.


Level 2 – Layout and Multiple Panels (Upper Beginner)
-----------------------------------------------------

Focus of this level:

- Using `Layout` to split the terminal into multiple areas.
- Displaying several panels at once.
- Showing different text in each panel.


Exercise 2.1 – Three‑Panel Layout Demo
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Learn basic vertical and horizontal `Layout`.

Description:

- Split the screen into three vertical sections:
  - Header (top).
  - Body (middle).
  - Footer (bottom).
- Inside the body, split into three columns:
  - Left, center, right panels.

Tasks:

1. Show different titles in each panel:
   - Header: "Rust TUI Lab".
   - Left: "Left Panel".
   - Center: "Center Panel".
   - Right: "Right Panel".
   - Footer: "Press q to quit".
2. Configure panel sizes:
   - Header: height 3 rows.
   - Footer: height 3 rows.
   - Body: fills the remaining space.
3. Configure body widths:
   - Left: 30% width.
   - Center: 40% width.
   - Right: 30% width.

Success criteria:

- The layout remains neat and proportional when resizing the terminal.
- Text in each panel is clearly visible.

Example command (if the binary is called `layout_demo`):

```bash
cargo run --bin layout_demo
```


Exercise 2.2 – Dynamic Info Panel
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Add simple dynamic behavior to a panel by updating text based on input.

Tasks:

1. Start from a layout similar to Exercise 2.1.
2. In one panel (for example the right panel), show a status text.
3. Add keys:
   - `1`: change status to "Mode 1 active".
   - `2`: change status to "Mode 2 active".
   - `3`: change status to "Mode 3 active".
4. Display the current status with a distinct style (for example a different color).

Success criteria:

- Pressing 1, 2, or 3 immediately updates the status text in the chosen panel.
- No panic or error occurs when other keys are pressed.


Level 3 – Stateful Widgets and Navigation (Intermediate)
--------------------------------------------------------

Focus of this level:

- Managing more complex application state.
- Using stateful widgets such as `List` with `ListState`.
- Handling navigation using keyboard arrows (up, down, select).


Exercise 3.1 – List with Highlight
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Build a list of items navigable with arrow keys.

Description:

- Display a list of items (for example menu entries or tasks).
- The currently selected item is highlighted and prefixed with a symbol (for example `>>`).
- Navigation:
  - Up arrow: move to the previous item.
  - Down arrow: move to the next item.
  - Wrap‑around behavior:
    - From last item to first.
    - From first item to last.

Tasks:

1. Define a struct to store:
   - The list state (`ListState`).
   - The item data, for example `Vec<String>`.
2. Implement methods:
   - `next()` to move down.
   - `previous()` to move up.
3. Render the list using `List` and `render_stateful_widget`.
4. Use a clear highlight style (different color and `Modifier::BOLD`).

Success criteria:

- Up and down navigation works correctly.
- The selected item is visually distinct.


Exercise 3.2 – Action on Selected Item
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Handle actions when the user presses `Enter` on the selected item.

Tasks:

1. Reuse the list from Exercise 3.1.
2. Add an info panel at the bottom of the screen that shows:
   - The name of the currently selected item.
   - A message when the item is confirmed with `Enter`.
3. When `Enter` is pressed:
   - Update the info panel, for example:
     - "You selected: <item name>".
4. Do not use `println!` to stdout for selection actions; show everything inside the TUI.

Success criteria:

- The info panel always reflects the currently selected item.
- After pressing `Enter`, the info panel correctly shows the selection message.


Level 4 – Simple Multi‑State Application (Upper Intermediate)
-------------------------------------------------------------

Focus of this level:

- Supporting multiple modes or screens in one application.
- Separating state logic from rendering logic.
- Handling a richer set of keyboard inputs.


Exercise 4.1 – Main Menu with Multiple Pages
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Create a main menu that can switch between pages or modes.

Description:

- The application has multiple "pages", for example:
  - "Home" page.
  - "Help" page.
  - "About" page.
- A menu on the left shows available pages.
- The right side shows content for the currently active page.

Tasks:

1. Define an enum for the pages or modes, for example:
   - `enum Page { Home, Help, About }`.
2. Define an application state that stores:
   - The current page.
   - Any other required data.
3. Implement page navigation:
   - Key `h`: switch to Home page.
   - Key `?`: switch to Help page.
   - Key `a`: switch to About page.
4. Render different content on the right side for each page.

Success criteria:

- Page transitions are smooth and clearly visible.
- The code is structured and easy to extend.


Exercise 4.2 – Simple Input Prompt
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Handle text input from the user inside the TUI.

Tasks:

1. Add an input prompt to one page (for example, the Home page):
   - An area on the screen that shows the string the user is typing.
2. Handle input for letters, digits, spaces, and the `Backspace` key.
3. Add keys:
   - `Enter`: confirm the input and show the result in another panel.
   - `Esc`: cancel input and clear the buffer.

Success criteria:

- The user can type, delete, and confirm text input.
- The input buffer displayed on screen is always in sync with what has been typed.


Level 5 – Mini Projects (Near Expert)
-------------------------------------

Focus of this level:

- Combine concepts from previous levels into a small but complete application.
- Organize code clearly so it is easy to maintain.


Exercise 5.1 – Todo TUI Application
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Build a simple todo application in the terminal.

Description:

- The application has:
  - A task list in the main panel.
  - An input panel for adding new tasks.
  - An indicator showing whether a task is completed or not.

Suggested features:

1. Add tasks:
   - Press a key (for example `n`) to enter "new task" input mode.
   - Type the task name and press `Enter` to add it to the list.
2. Toggle task completion:
   - Use up and down arrows to select a task.
   - Press `Space` or `Enter` to toggle completion.
3. Delete tasks:
   - Press a key (for example `d`) to delete the currently selected task.
4. Display:
   - Completed tasks marked with different symbols and style, for example:
     - `[x]` for done, `[ ]` for not done.
     - Dimmer color for completed tasks.

Success criteria:

- All basic todo operations (add, toggle complete, delete) work correctly.
- The UI remains responsive and does not panic during normal operations.


Exercise 5.2 – Mini Dashboard
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Goals:

- Build a layout that shows several types of information at once.

Description:

- The application has several panels, for example:
  - A log or message panel.
  - A short status panel (application state summary).
  - A simple "graph" panel using text bars.

Suggested features:

1. Timer or tick:
   - Use a tick mechanism (for example a timer via a channel) to update one panel
     periodically without user input.
2. Log:
   - Each time the user presses a certain key (for example `l`), append a new
     log entry to the log panel.
3. Text graph:
   - Show a numeric value that increases or decreases, visualized as a text bar, for example:
     - Value 3 displayed as `###`.
     - Value 5 displayed as `#####`.

Success criteria:

- The screen updates periodically even with no user input.
- Each panel shows consistent and meaningful information.

