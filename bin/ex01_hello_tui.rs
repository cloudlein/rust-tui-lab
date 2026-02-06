use std::io;
use color_eyre::Result;
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Paragraph};

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

    // wait for input
    loop {

        terminal.draw(|f| {
            let frame = f.area();

            let paragraph = Paragraph::new("Press q to quit")
                .block(Block::default().borders(Borders::ALL).title("Hello TUI"));

            f.render_widget(paragraph, frame)

        })?;

        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
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