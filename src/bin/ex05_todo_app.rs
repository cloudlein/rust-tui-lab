use chrono::{Duration, Local, NaiveDate};
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph};
use ratatui::{Frame, Terminal};
use std::io;

enum Page{
    Day,
    Input,
    History,
    Help
}

struct Task {
    date: NaiveDate,
    time: String,
    text: String,
    done: bool,
}

struct  InputBuffer {
    date: NaiveDate,
    time: String,
    text: String,
    focus: usize,
}

struct App {
    page: Page,
    day_offset: i32,
    tasks: Vec<Task>,
    list_state: ListState,
    input_buffer: InputBuffer,
}

impl App {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            page: Page::Day,
            day_offset: 0,
            tasks: vec![],
            list_state,
            input_buffer: InputBuffer {
                date: Local::now().date_naive(),
                time: "09:00".into(),
                text: String::new(),
                focus: 0,
            },
        }
    }

    fn day_items(&self) -> Vec<ListItem<'static>> {
        self.tasks
            .iter()
            .filter(|t| t.date == self.selected_day())
            .map(|t| {
                let prefix = if t.done { "✓" } else { " " };
                ListItem::new(format!("{} {}", prefix, t.text))
            })
            .collect()
    }

    fn selected_day(&self) -> NaiveDate {
        Local::now().date_naive() + Duration::days(self.day_offset as i64)
    }


    fn render_day_view(&mut self, frame: &mut Frame, dim: bool) {
        let container = frame.area();

        let style = if dim {
            Style::default().fg(Color::DarkGray)
        }else {
            Style::default()
        };

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
            Span::raw("◀ Previous "),
            Span::styled(
                format!("{label} · {}   ", formatted_date),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("Next ▶"),
        ]);

        let header_text = vec![
            title_text("TASK PLANNER", dim),
            header_line,
        ];


        let action_text = Line::from(
            Span::styled("Press n to add new task", default_style_text(dim))
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
            .style(style)
            .block(
                panel_block_with_padding_borders(2,0,0,0, Borders::LEFT | Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
            );

        let content_panel = List::new(self.day_items())
            .style(style)
            .block(
                panel_block_with_padding_borders(7, 0, 2, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );

        let action_panel = Paragraph::new(action_text)
            .style(style)
            .block(
                panel_block_with_padding_borders(2, 0, 0, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );

        let footer_panel = Paragraph::new(footer_text)
            .style(style)
            .block(
                panel_block_with_padding_borders(2, 0, 0, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );



        frame.render_widget(header_panel, vertical_layout[0]);
        frame.render_stateful_widget(content_panel, vertical_layout[1],  &mut self.list_state );
        frame.render_widget(action_panel, vertical_layout[2]);
        frame.render_widget(footer_panel, vertical_layout[3]);

    }



    fn render_input_view(&mut self, frame: &mut Frame) {

        self.render_day_view(frame, true);

        let popup_area = centered_react(80,45, frame.area());

        frame.render_widget(Clear, popup_area);

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),  // Header "NEW TASK"
                Constraint::Min(0),     // Form Fields
                Constraint::Length(3),  // Action Buttons
            ])
            .margin(1) // Margin dalam popup
            .split(popup_area);

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));


        frame.render_widget(popup_block, popup_area);

        let header_text = vec![
            Line::from(""),
            title_text("NEW TASKS PLANNER", false),
            Line::from(""),
            Line::from(
                Span::styled(
                    "Enter task details below", default_style_text(false),
                )
            )
        ];

        let header_panel_form = Paragraph::new(header_text)
            .style(default_style_text(false))
            .alignment(Alignment::Center);

        frame.render_widget(header_panel_form, layout[0]);

        let form_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(3)
            ]);


    }

    fn render_history_view(&self, frame: &mut Frame, dim: bool) {
        let container = frame.area();


        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(container);


        let header_text = vec![
            title_text("HISTORY", dim),
            Line::from(Span::styled(
                "Completed & past tasks", default_style_text(dim)
            )),
        ];



        let footer_text =  Line::from(vec![
            Span::raw("Esc "),
            Span::styled("Back to planner", Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let header_panel = Paragraph::new(header_text)
            .block(
                panel_block_with_padding_borders(2,0,0,0, Borders::LEFT | Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
            );

        // let content_panel = Paragraph::new(content_text)
        //     .block(
        //         panel_block_with_padding_borders(7, 0, 2, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
        //     );

        let footer_panel = Paragraph::new(footer_text)
            .block(
                panel_block_with_padding_borders(2, 0, 0, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );

        frame.render_widget(header_panel, vertical_layout[0]);
        //frame.render_widget(content_panel, vertical_layout[1]);
        frame.render_widget(footer_panel, vertical_layout[2]);

    }

    fn render_help_view(&self, frame: &mut Frame, dim: bool) {
        let container = frame.area();

        let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ]).split(container);

        let header_text = Line::from(title_text("HELP", dim));

        let content_text: Vec<Line> = vec![
            Line::from("←/→    Change day"),
            Line::from("↑/↓    Move selection"),
            Line::from("Enter  Toggle done"),
            Line::from("n      New task"),
            Line::from("d      Delete"),
            Line::from("h      History"),
            Line::from("q      Quit"),
        ];

        let footer_text =  Line::from(vec![
            Span::raw("Esc "),
            Span::styled("Back", Style::default().add_modifier(Modifier::BOLD)),
        ]);

        let header_panel = Paragraph::new(header_text)
            .block(
                panel_block_with_padding_borders(2,0,0,0, Borders::LEFT | Borders::RIGHT | Borders::TOP | Borders::BOTTOM)
            );

        let content_panel = Paragraph::new(content_text)
            .block(
                panel_block_with_padding_borders(7, 0, 2, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            );

        let footer_panel = Paragraph::new(footer_text)
            .block(
                panel_block_with_padding_borders(2, 0, 0, 0, Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
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
                Page::Day => app.render_day_view(f, false),
                Page::Input => app.render_input_view(f),
                Page::History => app.render_history_view(f, false),
                Page::Help => app.render_help_view(f, false),
                _ => {}
            }

        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q')  => break,
                    KeyCode::Char('n') => app.page = Page::Input,
                    KeyCode::Esc => app.page = Page::Day,
                    KeyCode::Char('h') => app.page = Page::History,
                    KeyCode::Char('?') => app.page = Page::Help,
                    _ => {}
                }

                match app.page {
                    Page::Day => {
                        match key.code {
                            KeyCode::Left => app.day_offset -= 1,
                            KeyCode::Right => app.day_offset += 1,
                            _ => {}
                        }
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

fn default_style_text(dim: bool) -> Style {
    if dim {
        Style::default().fg(Color::DarkGray)
    }else {
        Style::default().fg(Color::White)
    }
}

fn title_text(title: &str, dim: bool) -> Line<'static> {
    Line::from(
        Span::styled(title.to_string(), default_style_text(dim)
            .add_modifier(Modifier::BOLD)
        )
    )
}

fn panel_block_with_padding_borders(left: u16, right: u16, top: u16, bottom: u16, borders: Borders) -> Block<'static> {
    Block::default()
        .borders(borders)
        .padding(Padding::new(left, right, top, bottom))
}

fn centered_react(percent_x: u16, percent_y: u16, r: Rect) -> Rect  {
    let vertical_popup = Layout::vertical(
        [Constraint::Percentage(percent_x)])
        .flex(Flex::Center);

    let horizontal_popup = Layout::horizontal(
        [Constraint::Percentage(percent_y)])
        .flex(Flex::Center);

    let [r] = vertical_popup.areas(r);
    let [r] = horizontal_popup.areas(r);

    r
}