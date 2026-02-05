Level 4 – Simple Multi‑State Application (Upper Intermediate)
=============================================================

Focus of this level
--------------------

In Level 4 you will:

- Support multiple modes or screens in a single TUI application.
- Separate state logic from rendering logic.
- Handle a richer set of keyboard inputs.

Suggested binary names for this level:

- `src/bin/ex04_menu_pages.rs`
- `src/bin/ex04_input_prompt.rs`


Exercise 4.1 – Main Menu with Multiple Pages
--------------------------------------------

Goals
~~~~~

- Create a main menu that can switch between pages or modes.
- Organize your code to handle different screens cleanly.

Description
~~~~~~~~~~~

- The application has multiple "pages", for example:
  - "Home" page.
  - "Help" page.
  - "About" page.
- A menu on the left shows which pages are available.
- The right side shows content for the currently active page.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex04_menu_pages.rs`

2. Define an enum for the pages:

   - For example:
     - `enum Page { Home, Help, About }`

3. Define the application state struct:

   - Fields might include:
     - `current_page: Page`
     - Other data needed by specific pages.

4. Layout:

   - Use a horizontal layout:
     - Left: menu area (for example 20–30% of width).
     - Right: main content area.

5. Render the menu on the left:

   - Show the list of pages (Home, Help, About).
   - Highlight the currently selected page.
   - You can reuse a pattern similar to `StatefulList`, or simply highlight
     based on `current_page`.

6. Render the content on the right:

   - For `Page::Home`:
     - Show a short welcome message and maybe a short description of the app.
   - For `Page::Help`:
     - Display usage instructions, such as key bindings.
   - For `Page::About`:
     - Show a short text about yourself or about this Rust TUI Lab.

7. Keyboard input:

   - Key `h`: switch to Home page.
   - Key `?`: switch to Help page.
   - Key `a`: switch to About page.
   - `q` or `Esc`: quit the application.

Success criteria
~~~~~~~~~~~~~~~~

- Changing pages updates both the menu highlight and the content panel.
- Page transitions are smooth with no flickering or layout issues.
- The code is structured so that adding more pages later would be straightforward.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex04_menu_pages
```


Exercise 4.2 – Simple Input Prompt
----------------------------------

Goals
~~~~~

- Handle text input from the user inside the TUI.
- Store and display user‑entered text.

Description
~~~~~~~~~~~

- Add a text input prompt to one of your pages (for example Home).
- As the user types, characters appear in a "buffer" on screen.
- The user can confirm or cancel the input.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex04_input_prompt.rs`

2. Define the application state:

   - Include:
     - A `String` for the input buffer (what the user is currently typing).
     - A `String` for the last submitted value (could start as "None" or empty).
     - A boolean flag indicating whether you are currently in "input mode" or not.

3. Layout:

   - Use a vertical layout, for example:
     - Top: main content (can be simple text).
     - Middle: input prompt area.
     - Bottom: area showing the last submitted value.

4. Rendering:

   - In the input prompt area:
     - Show a label, for example "Input: ".
     - After the label, show the current contents of the input buffer.
     - Optionally, indicate whether you are in input mode (for example "[INSERT]").
   - In the bottom area:
     - Show the last submitted value, for example:
       - "Last submitted: <value>" or "Last submitted: None".

5. Keyboard input behavior:

   - Entering input mode:
     - Choose a key (for example `i`) that puts the app into input mode.
   - While in input mode:
     - Letter keys, digits, space:
       - Append the corresponding character to the input buffer.
     - `Backspace`:
       - Remove the last character from the input buffer if it is not empty.
     - `Enter`:
       - Copy the input buffer into the "last submitted" value.
       - Clear the input buffer.
       - Exit input mode.
     - `Esc`:
       - Clear the input buffer.
       - Exit input mode without updating the last submitted value.
   - Outside input mode:
     - Normal navigation keys (such as `q` or `Esc` to quit) should still work.

6. Quit behavior:

   - As in previous exercises, support `q` or `Esc` (outside input mode) to quit.

Success criteria
~~~~~~~~~~~~~~~~

- The input buffer accurately reflects what the user types.
- `Backspace` correctly removes characters from the buffer.
- `Enter` stores the buffer into the "last submitted" value and clears the buffer.
- `Esc` cancels input mode and clears the buffer without changing the last submitted value.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex04_input_prompt
```

