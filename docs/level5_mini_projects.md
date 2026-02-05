Level 5 – Mini Projects (Near Expert)
=====================================

Focus of this level
--------------------

In Level 5 you will:

- Combine concepts from previous levels into small but complete applications.
- Organize code clearly so it is easy to maintain and extend.

Suggested binary names for this level:

- `src/bin/ex05_todo_app.rs`
- `src/bin/ex05_dashboard.rs`


Exercise 5.1 – Todo TUI Application
-----------------------------------

Goals
~~~~~

- Build a simple todo application in the terminal.
- Practice managing a list of items with multiple actions.

Description
~~~~~~~~~~~

- The application should provide:
  - A task list in the main panel.
  - An input panel for adding new tasks.
  - An indicator showing whether a task is completed or not.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex05_todo_app.rs`

2. Define the core data structures:

   - A `Task` struct, for example:
     - `struct Task { title: String, done: bool }`
   - An application state struct that holds:
     - A `Vec<Task>` for all tasks.
     - A `ListState` for the selection in the list.
     - A `String` for the input buffer when adding new tasks.
     - A boolean flag indicating whether you are currently in "new task input" mode.

3. Layout:

   - Use a vertical layout or a two‑panel layout, for example:
     - Top: task list.
     - Bottom: input panel and help text.

4. Rendering:

   - Task list:
     - Display each task with a prefix:
       - `[ ]` for not done.
       - `[x]` for done.
     - Highlight the currently selected task.
   - Input panel:
     - When not in input mode:
       - Show a message like "Press n to add a new task".
     - When in input mode:
       - Show the current input buffer, for example:
         - "New task: <buffer>".
   - Help area:
     - Show key bindings, for example:
       - `Up/Down`: move selection
       - `Space/Enter`: toggle done
       - `n`: new task
       - `d`: delete task
       - `q/Esc`: quit

5. Keyboard input behavior:

   - Navigation:
     - Up arrow: move selection up in the task list.
     - Down arrow: move selection down.
   - Toggling completion:
     - `Space` or `Enter`:
       - Toggle the `done` state of the currently selected task.
   - Adding a new task:
     - Press `n` to enter "new task input" mode.
     - While in this mode:
       - Letters, digits, spaces: append to the input buffer.
       - `Backspace`: remove last character from the buffer if not empty.
       - `Enter`: create a new `Task` with:
         - `title` equal to the buffer content.
         - `done` = `false`.
         - Add it to the end of the list.
         - Clear the input buffer.
         - Exit input mode.
       - `Esc`: cancel input mode and clear the buffer.
   - Deleting a task:
     - Press `d` to delete the currently selected task (if any).
     - After deletion:
       - Adjust selection to a valid index (if the list is not empty).
   - Quitting:
     - `q` or `Esc` (when not in input mode) should quit the application.

6. State management notes:

   - Carefully handle the case where the list becomes empty.
   - Ensure you do not access the list with an out‑of‑bounds index after deletion.

Success criteria
~~~~~~~~~~~~~~~~

- You can add, toggle, and delete tasks without panics.
- The UI remains responsive and clearly shows the current selection and task states.
- The input mode for new tasks works smoothly.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex05_todo_app
```


Exercise 5.2 – Mini Dashboard
-----------------------------

Goals
~~~~~

- Build a TUI that shows several types of information at once.
- Practice handling both timed updates and user‑driven events.

Description
~~~~~~~~~~~

- The application has several panels, for example:
  - A log or message panel.
  - A short status panel (application state summary).
  - A simple "graph" panel using text bars.

Detailed tasks
~~~~~~~~~~~~~~

1. Create a new file:

   - `src/bin/ex05_dashboard.rs`

2. Define the application state:

   - A list of log messages (`Vec<String>`).
   - One or more numeric values representing metrics (for example an integer that goes up and down).
   - A tick counter or timestamp for periodic updates.

3. Layout:

   - Use a layout that clearly separates panels, for example:
     - Top: status panel.
     - Middle: graph panel.
     - Bottom: log panel.
   - Alternatively, use a more complex layout if you feel comfortable.

4. Status panel:

   - Show short information such as:
     - Current tick count.
     - Current value of your metric.
   - This panel should update as the internal state changes.

5. Graph panel:

   - Represent a numeric value as a bar made of characters, for example:
     - Value 3 -> `###`
     - Value 5 -> `#####`
   - Optionally, show a label such as "Value: 5".
   - You can use a simple text paragraph for this; a full chart is not necessary.

6. Log panel:

   - Maintain a list of log lines (for example most recent at the bottom).
   - Each time a certain key is pressed (for example `l`):
     - Append a new log entry, such as "User pressed L at tick X".
   - Keep the log from growing forever by optionally limiting its length (for example last 100 entries).

7. Tick/Timer behavior:

   - Use a tick mechanism to update parts of the UI at a fixed interval.
     - For example, update state every 200–500 ms.
   - Each tick may:
     - Increment or decrement your numeric value (perhaps randomly or following a pattern).
     - Update the status panel to reflect the new state.
   - Structure your event loop to handle both:
     - Timed events (ticks).
     - User events (key presses).

8. Keyboard input:

   - `l`: append a log entry.
   - Other keys as you wish (for example to manually change the numeric value).
   - `q` or `Esc`: quit the application.

Success criteria
~~~~~~~~~~~~~~~~

- The screen updates periodically even if you do not press any keys.
- The status and graph panels reflect the current state.
- New log entries appear when you press the designated key.
- The app exits cleanly without panics.

How to run it
~~~~~~~~~~~~~

From the project root:

```bash
cargo run --bin ex05_dashboard
```

