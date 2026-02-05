Level 2 – Layout and Multiple Panels (Upper Beginner)
=====================================================

Focus of this level
--------------------

In Level 2 you will:

- Use `Layout` to split the terminal into multiple areas.
- Show several panels on screen at the same time.
- Display different content in each panel.

Suggested binary names for this level:

- `src/bin/ex02_layout_demo.rs`
- `src/bin/ex02_dynamic_panel.rs`


Exercise 2.1 – Three‑Panel Layout Demo
--------------------------------------

Goals
~~~~~

- Learn basic vertical and horizontal `Layout` in `ratatui`.
- Understand how to subdivide the terminal into regions.

Description
~~~~~~~~~~~

- Split the screen vertically into:
  - Header (top).
  - Body (middle).
  - Footer (bottom).
- Inside the body, split horizontally into three columns:
  - Left, center, right panels.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex02_layout_demo.rs`

2. Draw the main layout:

   - Use a vertical `Layout` with three constraints:
     - Header: fixed height, for example 3 rows.
     - Body: `Constraint::Min(3)` or similar, to fill the remaining space.
     - Footer: fixed height, for example 3 rows.

3. Inside the body area:

   - Use a horizontal `Layout` with three constraints:
     - Left: 30% width.
     - Center: 40% width.
     - Right: 30% width.

4. Create a panel in each region:

   - Header:
     - Title: "Rust TUI Lab".
     - Some short descriptive text.
   - Left panel:
     - Title: "Left Panel".
     - Any placeholder text.
   - Center panel:
     - Title: "Center Panel".
     - Any placeholder text.
   - Right panel:
     - Title: "Right Panel".
     - Any placeholder text.
   - Footer:
     - Title: "Footer".
     - Text: "Press q or Esc to quit".

5. Add basic input handling:

   - `q` or `Esc` should quit the application.
   - You do not need more complex input handling yet for this exercise.

Success criteria
~~~~~~~~~~~~~~~~

- The layout looks proportional and remains usable when you resize the terminal.
- Content in header, body panels, and footer is clearly visible.
- Quitting the application restores the terminal state.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex02_layout_demo
```


Exercise 2.2 – Dynamic Info Panel
---------------------------------

Goals
~~~~~

- Add basic dynamic behavior to a panel by updating text based on input.
- Practice reading key events and updating state.

Description
~~~~~~~~~~~

- Reuse a layout similar to Exercise 2.1.
- Use one panel (for example the right panel) as a dynamic "status" or "info" panel.
- Pressing certain keys changes the text in that panel.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex02_dynamic_panel.rs`

2. Use a layout similar to `ex02_layout_demo`:

   - Header, body, footer vertically.
   - Three horizontal panels inside the body.

3. Choose one panel as the "status" panel:

   - For example, use the right panel.
   - Give it a title such as "Status".
   - Display a status text string inside this panel.

4. Add a small state struct:

   - Store the current status text as a `String`.
   - Initialize it with something like "No mode selected".

5. Handle key input to update the status:

   - `1`: set status text to "Mode 1 active".
   - `2`: set status text to "Mode 2 active".
   - `3`: set status text to "Mode 3 active".

6. Apply styling to the status text:

   - Choose a color to highlight the current mode, for example:
     - `Color::Green` for mode 1.
     - `Color::Yellow` for mode 2.
     - `Color::Red` for mode 3.
   - You may keep it simple and use the same color for all modes if you prefer,
     but some visual feedback is recommended.

7. Quit behavior:

   - Keep `q` or `Esc` to quit.

Success criteria
~~~~~~~~~~~~~~~~

- Pressing 1, 2, or 3 updates the status panel text immediately.
- Switching between modes updates the displayed text correctly.
- The rest of the layout remains visible and stable.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex02_dynamic_panel
```

