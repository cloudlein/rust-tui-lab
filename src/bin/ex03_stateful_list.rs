use std::io;
use color_eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Modifier;
use ratatui::style::{Color, Style};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};

struct StateFullList {
    state: ListState,
    items: Vec<String>
}

impl StateFullList {

    fn new() -> Self {
        let items = vec![
            "Item1".to_string(),
            "Item2".to_string(),
            "Item3".to_string(),
            "Item4".to_string(),
        ];

        let mut state = ListState::default();
        state.select(Some(0));

        Self { state, items }
    }


    fn next(&mut self) {
        // Take currently active index
        let i = match self.state.selected() {
            Some(i) => {
                // if index at the bottom, go back to the top ( looping )
                if i >= self.items.len() - 1 {
                    0
                }else {
                    i + 1
                }
            }

            // if not selected , default select
            None => 0,
        };

        // save new index to list state
        self.state.select(Some(i));
    }
    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                //  If at the top, jump to the bottom
                if i == 0 {
                    self.items.len() - 1
                }else {
                    i - 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // init instance

    let mut state = StateFullList::new();

    loop {
        terminal.draw(|f| {

            let frame = f.area();

            // make full screen
            let vertical_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(frame);

            // change all string to ListItem
            let items: Vec<ListItem> = state
                .items
                .iter()
                .map(|i| ListItem::new(i.as_str()))
                .collect();

            // create widget
            let list = List::new(items)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("List")
                )
                // style for selected items
                .highlight_style(
                    Style::default()
                        .fg(Color::White)
                        .bg(Color::LightBlue)
                        .add_modifier(Modifier::BOLD)
                )
                // symbol for selected item
                .highlight_symbol(">> ");

            // render with render stateful widget so list state available to read
            f.render_stateful_widget(list, vertical_layout[0], &mut state.state);
        })?;


        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Up => state.previous(),
                    KeyCode::Down => state.next(),
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