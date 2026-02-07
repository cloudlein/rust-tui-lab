use std::io;
use color_eyre::Result;
use crossterm::{event, execute};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Modifier;
use ratatui::style::{Color, Style, Stylize};
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

    // state mode
    let mut is_highlight_mode = false;


    // draw loop
    loop {

        terminal.draw(|f| {
            //  Define area to draw in (full screen here)
            let frame = f.area();


            if !is_highlight_mode {
                // title
                let title_normal = Line::from(" Normal Mode ".bold().white());

                // Create a paragraph widget
                let normal_text = vec![
                    Line::from("Muhammad Salahudin Al Ayyubi"),
                    Line::from("I want to make tools with rust"),
                    Line::from(vec![
                        Span::styled(
                            "Press 'h' to change mode to highlight mode",
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::BOLD),
                        )
                    ])
                ];

                let normal_paragraph = Paragraph::new(normal_text)
                    .block(Block::default()
                               .borders(Borders::ALL)
                               .title(title_normal)
                               .border_style(Style::default().fg(Color::Yellow)),
                    );

                // Render the widget
                f.render_widget(normal_paragraph, frame)

            }
            else  {
                // title highlight
                let title_highlight = Line::from(
                    Span::styled(
                        " Highlight Mode ",
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                );

                // Create a paragraph widget and give some style
                let styled_text = vec![
                    Line::from(vec![
                        Span::styled(
                            "Muhammad Salahudin Al Ayyubi",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                        )
                    ]),
                    Line::from(vec![
                        Span::styled(
                            "I want to make tools with rust",
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::ITALIC)
                        )
                    ]),
                    Line::from(vec![
                        Span::styled(
                            "Press 'h' again to change mode to normal mode",
                            Style::default()
                                .fg(Color::Red)
                                .add_modifier(Modifier::BOLD),
                        )
                    ])
                ];

                let style_paragraph = Paragraph::new(styled_text)
                    .block(Block::bordered()
                        .borders(Borders::ALL)
                        .title(title_highlight)
                        .border_style(Style::default().fg(Color::White)));

                // Render the widget
                f.render_widget(style_paragraph, frame)

            }

        })?;

        // wait for input
        if let Event::Key(key) = event::read()? {
           if key.kind == KeyEventKind::Press {
               match key.code {
                   KeyCode::Char('q') | KeyCode::Esc => break,
                   KeyCode::Char('h') => {
                       is_highlight_mode = !is_highlight_mode;
                   }
                   _ => {}
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