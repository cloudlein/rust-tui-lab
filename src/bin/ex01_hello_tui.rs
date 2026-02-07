use std::io;
use color_eyre::Result;
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::text::{Line, Span};

fn main() -> Result<()> {
    color_eyre::install()?;

    // Enter raw mode
    enable_raw_mode()?;

    // Enter alternate screen
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Create Terminal<CrosstermBackend<_>>
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // draw loop
    loop {

        terminal.draw(|f| {
           //  Define area to draw in (full screen here)
            let frame = f.area();

            // Create a paragraph widget

            let text = vec![
                Line::from("Muhammad Salahudin Al Ayyubi"),
                Line::from("I want to make tools with rust"),
            ];

            let paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Hello TUI"));

            // Render the widget
            f.render_widget(paragraph, frame)

        })?;

        // wait for input
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }else if key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }


    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}