use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Terminal;
use std::io;
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::text::{Line, Span};


struct AppState {
    mode: AppMode,
    display_text: String,
}

enum AppMode {
    Mode1,
    Mode2,
    Mode3,
    None,
}

fn main() -> Result<()> {
    color_eyre::install()?;


    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState{
        mode: AppMode::None,
        display_text: "No mode selected".to_string(),
    };

    loop {

        terminal.draw(|f| {
            let frame = f.area();

            let display_text = &state.display_text;
            let current_mode = &state.mode;


            let footer_text = vec![
                Line::from("press 1 to set status to 'Mode 1 active'"),
                Line::from("press 2 to set status to 'Mode 2 active'"),
                Line::from("press 3 to set status to 'Mode 3 active'"),
            ];

            let current_status_text =  Line::from(
                vec![
                    Span::styled(display_text,
                                 Style::default()
                                     .fg(
                                         match current_mode {
                                             AppMode::Mode1 => Color::Green,
                                             AppMode::Mode2 => Color::Yellow,
                                             AppMode::Mode3 => Color::Red,
                                             AppMode::None => Color::Reset,
                                         }
                                     )
                    .add_modifier(Modifier::BOLD)),
                ]
            );

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // header
                    Constraint::Min(3), // body
                    Constraint::Length(5), // footer
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

            let header_panel = Block::default()
                    .title(
                        Line::from("Ratatui UI Dynamic Panel")
                            .alignment(Alignment::Center)
                    )
                .borders(Borders::ALL)
                .style(Style::default()
                    .fg(Color::White)
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

            let right_panel = Paragraph::new(current_status_text)
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

            let footer_panel = Paragraph::new(footer_text)
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
                    KeyCode::Char('1') => {
                        state.mode = AppMode::Mode1;
                        state.display_text = "Mode 1 active".to_string();
                    },
                    KeyCode::Char('2') => {
                        state.mode = AppMode::Mode2;
                        state.display_text = "Mode 2 active".to_string();
                    },
                    KeyCode::Char('3') => {
                        state.mode = AppMode::Mode3;
                        state.display_text = "Mode 3 active".to_string();
                    },
                    KeyCode::Char('4') => {
                        state.mode = AppMode::None;
                        state.display_text = "No mode selected".to_string();
                    }
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