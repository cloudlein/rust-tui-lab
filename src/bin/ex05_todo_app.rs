use chrono::{Duration, Local};
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph};
use ratatui::{Frame, Terminal};
use std::io;

enum Page{
    Day,
    Input,
    History,
    Help
}


struct App {
    page: Page,
    day_offset: i32,
}

impl App {
    fn new() -> Self {
        Self {
            page: Page::Day,
            day_offset: 0,
        }
    }

    fn render_day_view(&self, frame: &mut Frame) {
        let container = frame.area();

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),   // header
                Constraint::Min(0),      // content
                Constraint::Length(2),   // action
                Constraint::Length(2),   // footer
            ])
            .split(container);

        let date = Local::now().date_naive() + Duration::days(self.day_offset as i64);
        let formatted_date = date.format("%d-%m-%Y").to_string();

        let label = match self.day_offset {
            0 => "Today",
            -1 => "Yesterday",
            1 => "Tomorrow",
            _ => "Selected Day",
        };

        let header_line = Line::from(vec![
            Span::raw("◀ Yesterday   "),
            Span::styled(
                format!("{label} · {}   ", formatted_date),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("Tomorrow ▶"),
        ]);

        let header_text = vec![
            title_text("TASK PLANNER"),
            header_line,
        ];

        let content_text = vec![
            Line::from(
                Span::styled("▸  09:00  Design clean architecture", default_style_text())
            ),
            Line::from(
                Span::styled("14:00  Fix GitHub connection", default_style_text())
            ),
            Line::from(
                Span::styled("✓  18:00  Finish Rust TUI Level 4", default_style_text())
            ),
        ];

        let action_text = Line::from(
            Span::styled("Press n to add new task", default_style_text())
        );

        let footer_text =  Line::from(vec![
            Span::raw("←/→ "),
            Span::styled("Change day", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("   n "),
            Span::styled("New", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("   h "),
            Span::styled("History", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("   ? "),
            Span::styled("Help", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("   q "),
            Span::styled("Quit", Style::default().add_modifier(Modifier::BOLD)),
        ]);


        let header_panel = Paragraph::new(header_text)
            .block(
                panel_block_with_padding_borders(2,0,0,0, Borders::LEFT | Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
            );

        let content_panel = Paragraph::new(content_text)
            .block(
                panel_block_with_padding_borders(7, 0, 2, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );

        let action_panel = Paragraph::new(action_text)
            .block(
                panel_block_with_padding_borders(2, 0, 0, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );

        let footer_panel = Paragraph::new(footer_text)
            .block(
                panel_block_with_padding_borders(2, 0, 0, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );



        frame.render_widget(header_panel, vertical_layout[0]);
        frame.render_widget(content_panel, vertical_layout[1]);
        frame.render_widget(action_panel, vertical_layout[2]);
        frame.render_widget(footer_panel, vertical_layout[3]);

    }

    fn render_input_view(&self, frame: &mut Frame) {
        let container = frame.area();

        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(container);

        let header_text = Line::from(
            Span::styled(
                "ADD NEW TASK", default_style_text()
            )
        );

        let content_text = vec![
            Line::from(
                Span::styled(
                    "Date: 2026-02-13", default_style_text()
                )
            ),
            Line::from(
                Span::styled(
                    "Time: 14:00", default_style_text()
                )
            ),
            Line::from(
                Span::styled(
                    "Title: build clean backend", default_style_text()
                )
            )
        ];

        let footer_text =  Line::from(vec![
            Span::raw("Tab"),
            Span::styled("Next", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("   Enter "),
            Span::styled("Save", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("   Esc "),
            Span::styled("Cancel", Style::default().add_modifier(Modifier::BOLD)),
        ]);


        let header_panel = Paragraph::new(header_text)
            .block(
              panel_block_with_padding_borders(2,0,0,0,Borders::LEFT | Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
            );

        let content_panel = Paragraph::new(content_text)
        .block(
            panel_block_with_padding_borders(2,0,4,0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        );

        let footer_panel = Paragraph::new(footer_text)
        .block(
          panel_block_with_padding_borders(2,0,0,0,Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        );

        frame.render_widget(header_panel, vertical_layout[0]);
        frame.render_widget(content_panel, vertical_layout[1]);
        frame.render_widget(footer_panel, vertical_layout[2]);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| {

            match app.page {
                Page::Day => app.render_day_view(f),
                Page::Input => app.render_input_view(f),
                _ => {}
            }

        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('n') => app.page = Page::Input,
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

fn default_style_text() -> Style {
    Style::default().fg(Color::White)
}

fn title_text(title: &str) -> Line<'static> {
    Line::from(
        Span::styled(title.to_string(), default_style_text()
            .add_modifier(Modifier::BOLD)
        )
    )
}

fn panel_block_with_padding_borders(left: u16, right: u16, top: u16, bottom: u16, borders: Borders) -> Block<'static> {
    Block::default()
        .borders(borders)
        .padding(Padding::new(left, right, top, bottom))
}
