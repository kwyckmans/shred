use tui::{widgets::{ListState, ListItem, Borders, List, Block}, backend::Backend, Frame, layout::{Rect, Layout, Direction, Constraint}, style::{Style, Color, Modifier}};

const MENU_ITEMS: [&str; 4] = ["Practice", "Songbook", "Exercises", "Routines"];

pub struct UI {
    menu_state: ListState,
}

impl UI {
    pub fn new() -> UI {
        UI {
            menu_state: ListState::default(),
        }
    }

    pub fn select_next_menu_entry(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i >= MENU_ITEMS.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    pub fn select_prev_menu_entry(&mut self) {
        let i = match self.menu_state.selected() {
            Some(i) => {
                if i == 0 {
                    MENU_ITEMS.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.menu_state.select(Some(i));
    }

    fn render_menu<B>(&mut self, f: &mut Frame<B>, area: Rect)
    where
        B: Backend,
    {
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

        let menu_items = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Menu"))
            .highlight_style(
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(menu_items, area, &mut self.menu_state);
    }

    pub fn render<B>(&mut self, f: &mut Frame<B>, render_help: bool)
    where
        B: Backend,
    {
        let menus: Vec<fn(&mut Frame<B>, Rect)> = vec![
            draw_first_tab,
            draw_second_tab,
            draw_first_tab,
            draw_first_tab,
            draw_first_tab,
        ];

        let mut chunks = vec![];

        if render_help {
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

        self.render_menu(f, chunks[0]);

        // self.menu.render(f, chunks[0]);
        match self.menu_state.selected() {
            Some(i) => menus[i](f, chunks[1]),
            None => draw_first_tab(f, chunks[1]),
            _ => unreachable!(),
        }
    }
}

fn draw_first_tab<B>(f: &mut Frame<B>, area: Rect)
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

fn draw_second_tab<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area);

    let block = Block::default().title("Songbook").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
}
