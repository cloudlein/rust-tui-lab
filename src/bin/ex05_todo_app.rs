use color_eyre::Result;
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::{Frame, Terminal};
use std::io;
use chrono::{Duration, Local};
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::Paragraph;

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
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(2),
                Constraint::Length(2),
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

        let header = Paragraph::new(header_text);
        frame.render_widget(header, vertical_layout[0]);

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
                _ => {}
            }

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

fn default_style_text() -> Style {
    Style::default().fg(Color::White)
}

fn title_text(title: &str) -> Line<'static> {
    Line::from(
        Span::styled(title.to_string(), default_style_text()
            .add_modifier(Modifier::BOLD))
    )
}
