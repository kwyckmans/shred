use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, cursor::MoveDown,
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
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

struct App {
    help_visible: bool,
    menu_items: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            help_visible: false,
            menu_items: vec![
                String::from("Practice"),
                String::from("Songbook"),
                String::from("Exercises"),
                String::from("Routines"),
            ],
        }
    }

    fn toggle_help(&mut self) {
        self.help_visible = !self.help_visible;
    }

    // fn handle_input(&mut self, key: KeyEvent) -> Result<(), io::Error> {
    //     match key.code {
    //         KeyCode::Down => self.ui.select_next_menu_entry(),
    //         KeyCode::Up => self.ui.select_prev_menu_entry(),
    //         KeyCode::Char('h') => self.toggle_help(),
    //         _ => {}
    //     }

    //     Ok(())
    // }
}

const MENU_ITEMS: [&str;4] = ["Practice", "Songbook", "Exercises", "Routines"];

struct Menu<'a> {
    menu_items: List<'a>,
    menu_state: ListState,
}

impl<'a> Menu<'a> {
    fn new() -> Menu<'a> {
        let items: Vec<ListItem> = MENU_ITEMS
            .iter()
            // I'm moving the my menu entries from my StatefulList into ListItems.
            // This means that ownership of `i` moves to `ListItem`.
            //
            // Map has a type of &String
            // Why? -> See work slack.
            // ListItem::new(some_var) doesn't accept that reference directly because &String does not implement into.
            // An explanation, that I don't understand, can be found at https://stackoverflow.com/questions/45126120/if-intostring-is-not-implemented-for-string-why-are-these-implementation.
            // If I derefernce i, I have type String, but that's owned by my StatefulList. It cannot move it, because
            // it doesn't implement the copy operator.
            // I can convert from String to &str
            .map(|i| ListItem::new(*i).style(Style::default().fg(Color::White)))
            .collect();

        // let items =

        Menu {
            menu_items: List::new(items),
            menu_state: ListState::default()
        }
        // Menu {
        //     menu_items: List::new(menu_items)
        //         .block(Block::default().borders(Borders::ALL).title("Menu"))
        //         .highlight_style(
        //             Style::default()
        //                 .bg(Color::White)
        //                 .fg(Color::Black)
        //                 .add_modifier(Modifier::BOLD),
        //         ),
        //     menu_state: ListState::default(),
        // }
    }

    fn next(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i >= 4 - 1 {
                    // if i >= self.menu_state.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    4 - 1
                    // self.menu_items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    fn render<B>(self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
        // f.render_widget(self.menu_items, area);
        f.render_stateful_widget(self.menu_items, area, &mut ListState::default());
    }
}


struct UI<'a> {
    menu: Menu<'a>,
}

impl<'a> UI<'a> {
    fn new(m: Menu<'a>) -> UI<'a> {
        UI {
            menu: m,
        }
    }

    fn select_next_menu_entry(&mut self) {
        self.menu.next();
    }

    fn select_prev_menu_entry(&mut self) {
        self.menu.previous();
    }

    fn render<B>(&self, f: &mut Frame<B>, app: &App)
    where
        B: Backend,
    {
        let menus: Vec<fn(&mut Frame<B>, &App, Rect)> = vec![
            draw_first_tab,
            draw_second_tab,
            draw_first_tab,
            draw_first_tab,
            draw_first_tab,
        ];

        let mut chunks = vec![];

        if app.help_visible {
            chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(f.size());
        } else {
            chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(85)].as_ref())
                .split(f.size());
        }

        // TODO: This is the sole reason app needs to be mutable. Which seem unnecessary, we only
        //   keep this stateful list to highlight a menu, ie fully a visual thing
        // f.render_stateful_widget(self.menu, chunks[0], &mut self.menu);

        self.menu.render(f, chunks[0]);
        // match self.menu_items.state.selected() {
        //     Some(i) => menus[i](f, app, chunks[1]),
        //     None => draw_first_tab(f, app, chunks[1]),
        //     _ => unreachable!(),
        // }
    }
}

// fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
//     // let size = f.size();

// }

fn draw_first_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
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

fn draw_second_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let block = Block::default().title("Songbook").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, mut ui: UI) -> io::Result<()> {
    loop {
        // Draw the ui defined in the ui function with the data contained in App.
        // TODO: ui should maybe be a struct, instead of just a method? It's getting a tad complicated.
        // TODO: Why is app mutable when passing it to draw? It should be mutable when handling input, but not
        //   when drawing?
        // terminal.draw(|f| ui.render(f, &app))?;
        terminal.draw(

            |f| ui.render(f, &app)
        );
        // ui.render(f, &app);
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => ui.select_next_menu_entry(),
                KeyCode::Up => ui.select_prev_menu_entry(),
                KeyCode::Char('h') => app.toggle_help(),
                _ => {}
            }

            // let res = app.handle_input(key);

            // match res {
            //     Ok(_) => continue,
            //     Err(_) => return Result::Err(res.err().unwrap()),
            // }
        }
    }
}

// Application entry point. We start here.
fn main() -> Result<(), io::Error> {
    // Disables a bunch of options in the terminal this app runs in, so you can do more stuff with it.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Crossterm supports 2 screens, the main screen and the alternate screen.
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let menu: Menu = Menu::new();
    let ui = UI::new(menu);
    let res = run_app(&mut terminal, app, ui);

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
