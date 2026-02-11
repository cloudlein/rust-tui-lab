use std::io;
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};


struct App {
    input: String,
    character_index: usize,
    input_mode: InputMode,
    message: Vec<String>,
}

enum InputMode {
    Normal,
    Editing
}

impl App {

   const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            message: Vec::new(),
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    
}


fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;


    loop {
        terminal.draw(| f| {
            let frame = f.area();

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(7),
                    Constraint::Length(3),
                    Constraint::Length(3),
                ])
                .split(frame);

            let header_text = vec![
                Line::from(
                    Span::styled(
                        "Input Prompt Exercise",
                        default_title_text_style()
                    ),
                ).alignment(Alignment::Center),
                Line::from(
                    Span::styled(
                        "Press [i] to start typing",
                        default_style_text()
                    )
                ),
                Line::from(
                    Span::styled(
                        "Press [q] to quit",
                        default_style_text()
                    )
                )
            ];


            let input_text = Line::from(Span::styled(
                "[ INSERT ] Input: ",
                default_style_text()
            ));

            let submit_text = Line::from(
                Span::styled("Last Submitted: ", default_style_text())
            );

            let header_panel = Paragraph::new(header_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .padding(Padding::new(4, 4, 0, 0))
                );

            let input_panel = Paragraph::new(input_text)
                .block(
                    Block::default()
                    .borders(Borders::ALL)
                );

            let submit_panel = Paragraph::new(submit_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
            );;

            f.render_widget(header_panel, vertical_layout[0]);
            f.render_widget(input_panel, vertical_layout[1]);
            f.render_widget(submit_panel, vertical_layout[2]);

        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => break,
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


fn default_style_text() -> Style{
    Style::default()
        .fg(Color::White)
}

fn default_title_text_style() -> Style {
    Style::default()
    .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}