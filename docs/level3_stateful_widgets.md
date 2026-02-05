Level 3 – Stateful Widgets and Navigation (Intermediate)
========================================================

Focus of this level
--------------------

In Level 3 you will:

- Manage more complex application state.
- Use stateful widgets such as `List` with `ListState`.
- Handle navigation with keyboard arrows (up, down, select).

Suggested binary names for this level:

- `src/bin/ex03_stateful_list.rs`
- `src/bin/ex03_list_action.rs`


Exercise 3.1 – List with Highlight
----------------------------------

Goals
~~~~~

- Build a list of items that can be navigated with the arrow keys.
- Learn how to use `List` with `ListState` in `ratatui`.

Description
~~~~~~~~~~~

- Display a vertical list of items (for example menu entries or tasks).
- One item is always selected and visually highlighted.
- The user can move the selection using the arrow keys.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex03_stateful_list.rs`

2. Define the list state:

   - Create a struct, for example:
     - `struct StatefulList { state: ListState, items: Vec<String> }`
   - Initialize it with several items, such as:
     - "Item 1", "Item 2", "Item 3", "Item 4".
   - Ensure that when there are items, one of them is selected by default.

3. Implement navigation methods:

   - `next()`:
     - Move the selection down by one.
     - If the current item is the last one, wrap around to the first item.
   - `previous()`:
     - Move the selection up by one.
     - If the current item is the first one, wrap around to the last item.

4. Render the list:

   - Use `List` from `ratatui::widgets` to display the items.
   - Use `render_stateful_widget` to connect `List` with your `ListState`.
   - Apply a highlight style for the selected item, for example:
     - Different color (e.g. `Color::Yellow`).
     - `Modifier::BOLD`.
   - Optionally, use a highlight symbol like `">> "` in front of the selected item.

5. Handle keyboard input:

   - Up arrow: call `previous()`.
   - Down arrow: call `next()`.
   - `q` or `Esc`: quit the application.

Success criteria
~~~~~~~~~~~~~~~~

- Up and down navigation works correctly and wraps around the list.
- The selected item is clearly highlighted.
- The application cleans up the terminal correctly on exit.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex03_stateful_list
```


Exercise 3.2 – Action on Selected Item
--------------------------------------

Goals
~~~~~

- React to the selected item when the user presses `Enter`.
- Display information about the selected item inside the TUI.

Description
~~~~~~~~~~~

- Extend the list interaction from Exercise 3.1.
- Add an info panel at the bottom that shows:
  - Which item is currently selected.
  - A message when the item is confirmed with `Enter`.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex03_list_action.rs`

2. Reuse or adapt the `StatefulList` structure:

   - Keep the navigation behavior from Exercise 3.1.
   - Ensure the list and selection logic are cleanly implemented.

3. Add a new piece of state for the info panel:

   - Store:
     - The name of the currently selected item (or its index).
     - The last "confirmed" item name as a string (initially something like "None").

4. Layout:

   - Use a vertical layout with:
     - Top section: the list itself.
     - Bottom section: the info panel.

5. Rendering:

   - In the top section:
     - Render the list as in Exercise 3.1.
   - In the bottom section (info panel):
     - Display:
       - "Current selection: <item name or index>".
       - "Last confirmed: <item name or 'None'>".

6. Keyboard input:

   - Up arrow: move selection up.
   - Down arrow: move selection down.
   - `Enter`:
     - Update the "last confirmed" item to the currently selected item.
   - `q` or `Esc`: quit the application.

7. Do not use `println!` for UI messages:

   - All feedback about selection and confirmation should be shown inside the TUI,
     not printed to the regular stdout.

Success criteria
~~~~~~~~~~~~~~~~

- The info panel always reflects the currently selected item.
- Pressing `Enter` updates the "last confirmed" item correctly.
- The list navigation behavior remains correct from Exercise 3.1.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex03_list_action
```

