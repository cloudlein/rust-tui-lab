// Standard library for input/output to the terminal
use std::io;

// A library that gives better error messages (nice stack traces)
use color_eyre::Result;

// Crossterm handles keyboard events and terminal control
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode,
    EnterAlternateScreen, LeaveAlternateScreen
};
use crossterm::{event, execute};

// Ratatui is the TUI (Text User Interface) framework
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Position};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

// Used to calculate how wide a string is in the terminal
// This is important for Unicode characters (like emoji or accented letters)
use unicode_width::UnicodeWidthStr;


// APPLICATION STATE
// This struct stores ALL data of our app
// Think of it as the "brain" of the program
struct App {
    input: String,              // The text the user is currently typing
    character_index: usize,    // Cursor position (in characters, not bytes!)
    input_mode: InputMode,     // Are we typing or not?
    last_submitted: Option<String>, // Last confirmed input
}

// This enum represents two modes of the app
enum InputMode {
    Normal,   // User is not typing
    Editing,  // User is typing
}



// APPLICATION LOGIC
impl App {

    // Create a new App with default values
    const fn new() -> Self {
        Self {
            input: String::new(),          // start with empty input
            input_mode: InputMode::Normal,// start in Normal mode
            last_submitted: None,          // nothing submitted yet
            character_index: 0,            // cursor at start
        }
    }

    // Move cursor one character to the left
    fn move_cursor_left(&mut self) {
        // saturating_sub prevents going below 0
        let new_pos = self.character_index.saturating_sub(1);

        // clamp makes sure cursor stays inside valid range
        self.character_index = self.clamp_cursor(new_pos);
    }

    // Move cursor one character to the right
    fn move_cursor_right(&mut self) {
        let new_pos = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(new_pos);
    }

    // Insert a new character at the cursor position
    fn enter_char(&mut self, new_char: char) {
        // Convert from character index to byte index
        let index = self.byte_index();

        // Insert the character into the string
        self.input.insert(index, new_char);

        // Move cursor after the new character
        self.move_cursor_right();
    }

    // Convert character index → byte index
    // Rust strings are UTF-8, so characters may use more than 1 byte
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()       // get all character positions
            .map(|(i, _)| i)      // take only the byte index
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    // Delete the character BEFORE the cursor
    fn delete_char(&mut self) {
        if self.character_index == 0 {
            return; // nothing to delete
        }

        // Get current byte index
        let idx = self.byte_index();

        // Find previous character
        let prev_char = self.input[..idx].chars().last().unwrap();
        let char_len = prev_char.len_utf8();

        // Remove that character from the string
        let from = idx - char_len;
        self.input.replace_range(from..idx, "");

        // Move cursor left
        self.move_cursor_left();
    }

    // Reset cursor back to start
    fn rest_cursor(&mut self) {
        self.character_index = 0;
    }

    // Save input as last_submitted and reset buffer
    fn submit_input(&mut self) {
        if !self.input.is_empty() {
            self.last_submitted = Some(self.input.clone());
        }

        self.input.clear();
        self.rest_cursor();
        self.input_mode = InputMode::Normal;
    }

    // Prevent cursor from going outside valid range
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        let max = self.input.chars().count();
        new_cursor_pos.clamp(0, max)
    }
}


// MAIN PROGRAM
fn main() -> Result<()> {
    // Install better error handling
    color_eyre::install()?;

    // Enable raw mode so keys are read instantly
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    // MAIN LOOP
    loop {
        terminal.draw(|f| {
            let frame = f.area();

            // Split screen into 3 vertical parts
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(7),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(frame);

            // HEADER TEXT
            let header_text = vec![
                Line::from(Span::styled("Input Prompt Exercise", default_title_text_style()))
                    .alignment(Alignment::Center),
                Line::from(Span::styled("Press [i] to start typing", default_style_text())),
                Line::from(Span::styled("Press [q] to quit", default_style_text())),
            ];

            let mode_label = match app.input_mode {
                InputMode::Normal => "",
                InputMode::Editing => "[INSERT]",
            };

            let input_text = Line::from(vec![
                Span::styled(format!("{mode_label} Input: "), default_style_text()),
                Span::raw(app.input.as_str()),
            ]);

            let last = app.last_submitted.as_deref().unwrap_or("None");

            let submit_text = Line::from(vec![
                Span::styled("Last Submitted: ", default_style_text()),
                Span::raw(last),
            ]);

            // DRAW PANELS
            let header_panel = Paragraph::new(header_text)
                .block(Block::default().borders(Borders::ALL).padding(Padding::new(4, 4, 0, 0)));

            let input_panel = Paragraph::new(input_text)
                .block(Block::default().borders(Borders::ALL));

            let submit_panel = Paragraph::new(submit_text)
                .block(Block::default().borders(Borders::ALL));

            f.render_widget(header_panel, layout[0]);
            f.render_widget(input_panel, layout[1]);
            f.render_widget(submit_panel, layout[2]);

            // Show cursor only when typing
            if let InputMode::Editing = app.input_mode {
                let prefix = format!("{mode_label} Input: ");
                let prefix_width = prefix.width() as u16;

                let input_area = layout[1];
                let cursor_x = app.input[..app.byte_index()].width() as u16;

                f.set_cursor_position(Position::new(
                    input_area.x + 1 + prefix_width + cursor_x,
                    input_area.y + 1,
                ));
            }
        })?;

        // READ KEYBOARD INPUT
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                    KeyCode::Char('q') => break,
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_input(),
                    KeyCode::Char(c) => app.enter_char(c),
                    KeyCode::Backspace => app.delete_char(),
                    KeyCode::Left => app.move_cursor_left(),
                    KeyCode::Right => app.move_cursor_right(),
                    KeyCode::Esc => {
                        app.input.clear();
                        app.rest_cursor();
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    // CLEANUP TERMINAL
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}


// STYLES
// Normal text style
fn default_style_text() -> Style {
    Style::default().fg(Color::White)
}

// Title style
fn default_title_text_style() -> Style {
    Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}
