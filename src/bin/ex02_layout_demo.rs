use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Modifier;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;
use std::io;

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    loop {
        terminal.draw(|f| {
           let frame = f.area();

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // header
                    Constraint::Min(3), // body
                    Constraint::Length(3), // footer
                ])
                .split(frame);

            let horizontal_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30), // left
                    Constraint::Percentage(40), // center
                    Constraint::Percentage(30), // right
                ])
                .split(vertical_layout[1]);

            let header_panel = Paragraph::new("this lab include all exercise to master ratatui from newbie to expert")
                .wrap(Wrap { trim: true })
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Rust TUI Lab")
                    .style(Style::default()
                        .fg(Color::LightCyan)
                    .add_modifier(Modifier::BOLD)
                    )
                );

            let center_panel = Paragraph::new("This is a center panel content")
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL)
                    .title("Center Panel")
                    .style(Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                    )
                );

            let right_panel = Paragraph::new("This is a right panel content")
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL)
                    .title("Right Panel")
                    .style(Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                    )
                );

            let left_panel = Paragraph::new("This is a left panel content")
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL)
                    .title("Left Panel")
                    .style(Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                    )
                );

            let footer_panel = Paragraph::new("Press q or Esc to quit")
                .wrap(Wrap { trim: true })
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Footer")
                    .style(Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD)
                    )
                );


            f.render_widget(header_panel, vertical_layout[0]);
            f.render_widget(left_panel, horizontal_layout[0]);
            f.render_widget(center_panel, horizontal_layout[1]);
            f.render_widget(right_panel, horizontal_layout[2]);
            f.render_widget(footer_panel, vertical_layout[2]);


        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}