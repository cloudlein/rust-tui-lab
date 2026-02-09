use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;
use std::io;

enum Page {
    Home,
    Help,
    About,
}
struct AppPage {
    current_page: Page,
}

impl AppPage {
    fn new() -> Self {
        let mut current_page = Page::Home;

        Self { current_page }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_page = AppPage::new();

    loop {
        terminal.draw(| f| {
            let frame = f.area();

            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(3),
                    Constraint::Length(3),
                ])
                .split(frame);

            let horizontal_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ])
                .split(vertical_layout[1]);

            let header_title = Line::from(
                Span::styled(
                    "Rust TUI Lab – Menu Pages",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                )
            ).alignment(Alignment::Center);


            let menu_text = vec![
                Line::from(
                    Span::styled(
                        "MENU",
                        Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                    )
                ).alignment(Alignment::Center),
                Line::from(""),
                Line::from(
                    Span::styled(
                        "Home",
                        Style::default()
                            .fg(Color::White)
                    )
                ).alignment(Alignment::Center),
                Line::from(
                    Span::styled(
                        "Help",
                        Style::default()
                            .fg(Color::White)
                    )
                ).alignment(Alignment::Center),
                Line::from(
                    Span::styled(
                        "About",
                        Style::default()
                            .fg(Color::White)
                    )
                ).alignment(Alignment::Center),
                Line::from(""),
            ];


            let page_lines: Vec<Line> = match app_page.current_page {
                Page::Home => vec![
                    Line::from("Welcome to the Rust TUI Lab."),
                    Line::from(""),
                    Line::from("This small application demonstrates how a multi-page"),
                    Line::from("terminal interface can be built using ratatui."),
                    Line::from(""),
                    Line::from("Use the menu on the left or the keyboard shortcuts."),
                ],

                Page::Help => vec![
                    Line::from("Help — Key Bindings"),
                    Line::from(""),
                    Line::from("h   : Go to Home page"),
                    Line::from("?   : Open Help page"),
                    Line::from("a   : Open About page"),
                    Line::from("q   : Quit the application"),
                    Line::from("Esc : Quit the application"),
                    Line::from(""),
                    Line::from("Navigate through the app using your keyboard."),
                ],

                Page::About => vec![
                    Line::from("About This Project"),
                    Line::from(""),
                    Line::from("This TUI was built as part of a learning journey"),
                    Line::from("to understand layout, state management, and rendering"),
                    Line::from("in terminal applications using Rust and ratatui."),
                    Line::from(""),
                    Line::from("Created as a hands-on lab to explore UI in terminal."),
                ],
            };

            let mut content_lines = vec![
                Line::from(
                    Span::styled(
                        "CONTENT",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    )
                ).alignment(Alignment::Center),
                Line::from(""),
            ];

            content_lines.extend(page_lines);

            content_lines.push(Line::from(""));



            let footer_text = Line::from(vec![
                Span::raw("h:Home   "),
                Span::raw("?:Help   "),
                Span::raw("a:About   "),
                Span::raw("q/esc:Quit"),
            ]).alignment(Alignment::Center);


            let header_panel = Paragraph::new(header_title)
                    .block(Block::default()
                    .borders(Borders::ALL)
                );

            let menu_panel = Paragraph::new(menu_text)
                .block(Block::default()
                .borders(Borders::ALL)
            );


            let content_panel = Paragraph::new(content_lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL),
                );


            let footer_panel = Paragraph::new(footer_text)
                .block(Block::default()
                .borders(Borders::ALL));

            f.render_widget(header_panel, vertical_layout[0]);
            f.render_widget(menu_panel, horizontal_layout[0]);
            f.render_widget(content_panel, horizontal_layout[1]);
            f.render_widget(footer_panel, vertical_layout[2]);

        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('h')  => app_page.current_page = Page::Home,
                    KeyCode::Char('?') => app_page.current_page = Page::Help,
                    KeyCode::Char('a') => app_page.current_page = Page::About,
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