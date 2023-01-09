// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Tabs},
    Frame, Terminal,
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };

        list.state.select(Some(0));

        list
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    items: StatefulList<&'a str>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(vec![
                "Item0",
                "Item1",
                "Item2",
                "Item3"
                ]),
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Disables a bunch of options in the terminal this app runs in, so you can do more stuff with it.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Crossterm supports 2 screens, the main screen and the alternate screen.
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    // restore terminal. Is this after quitting your app? Unsure.
    // Totally unclear, but I'm not handling user input yet, so, maybe that'll come later.
    // TODO: Figure out when in the lifecycle this is supposed to be called, and what happens
    //  if I don't.
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(), // TODO: Why is this in the terminal backend, but the first execute is on stdout?
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(f.size());

    let menu_entries: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            ListItem::new(String::from(*i)).style(Style::default().fg(Color::White))
        })
        .collect();

    let items = List::new(menu_entries)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .highlight_style(
            Style::default()
                .bg(Color::White)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, chunks[0], &mut app.items.state);

    //   ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))

    //             }
    //         )

    // let main_menu = List::new(menu_entries);

    // f.render_widget(main_menu, chunks[0]);

    match app.items.state.selected() {
        Some(0) => draw_first_tab(f, app, chunks[1]),
        Some(1) => draw_second_tab(f, app, chunks[1]),
        Some(2) => draw_first_tab(f, app, chunks[1]),
        Some(3) => draw_second_tab(f, app, chunks[1]),
        _ => unreachable!(),
    }
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let block = Block::default()
        .title("Practice routine")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &mut App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let block = Block::default().title("Songbook").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Left => app.items.unselect(),
                KeyCode::Down => app.items.next(),
                KeyCode::Up => app.items.previous(),
                _ => {}
            }
        }
    }
}
