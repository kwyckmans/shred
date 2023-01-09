
// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }


use std::{io, thread, time::Duration};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, self, Event, KeyCode}};
use tui::{
    backend::{CrosstermBackend, Backend}, Terminal, widgets::{Block, Borders, Tabs}, Frame, layout::{Direction, Layout, Constraint}, style::{Style, Color, Modifier}, text::{Spans, Span},
};

struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["Tab0", "Tab1", "Tab2", "Tab3"],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
         .direction(Direction::Vertical)
         .margin(5)
         .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
         .split(size);


    let block = Block::default().style(Style::default().bg(Color::White).fg(Color::Black));

    f.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black),
        );

    f.render_widget(tabs, chunks[0]);

    let inner = match app.index {
        0 => Block::default().title("Inner 0").borders(Borders::ALL),
        1 => Block::default().title("Inner 1").borders(Borders::ALL),
        2 => Block::default().title("Inner 2").borders(Borders::ALL),
        3 => Block::default().title("Inner 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
    //  let block = Block::default()
    //       .title("Block 2")
    //       .borders(Borders::ALL);
    //  f.render_widget(block, chunks[1]);
 }

// fn main() -> Result<(), Box<dyn Error>> {    
//     // let args = Cli::parse();

//     // let content = std::fs::read_to_string(&args.path);
//     // let content = match content {
//     //     Ok(content) => content,
//     //     Err(error) => {
//     //         panic!("Could not read provided file {:?}", error)
//     //     }
//     // };

//     // for line in content.lines() {
//     //     if line.contains(&args.pattern) {
//     //         println!("{}", line)
//     //     }
//     // }

//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;

//     // create app and run it
//     let res = run_app(&mut terminal);

//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;

//     if let Err(err) = res {
//         println!("{:?}", err)
//     }

//     Ok(())
// }

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
}

// fn ui<B: Backend>(f: &mut Frame<B>) {
//     // Wrapping block for a group
//     // Just draw the block and the group on the same area and build the group
//     // with at least a margin of 1
//     let size = f.size();

//     // Surrounding block
//     let block = Block::default()
//         .borders(Borders::ALL)
//         .title("Main block with round corners")
//         .title_alignment(Alignment::Center)
//         .border_type(BorderType::Rounded);
//     f.render_widget(block, size);

//     let chunks = Layout::default()
//         .direction(Direction::Vertical)
//         .margin(4)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//         .split(f.size());

//     // Top two inner blocks
//     let top_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//         .split(chunks[0]);

//     // Top left inner block with green background
//     let block = Block::default()
//         .title(vec![
//             Span::styled("With", Style::default().fg(Color::Yellow)),
//             Span::from(" background"),
//         ])
//         .style(Style::default().bg(Color::Green));
//     f.render_widget(block, top_chunks[0]);

//     // Top right inner block with styled title aligned to the right
//     let block = Block::default()
//         .title(Span::styled(
//             "Styled title",
//             Style::default()
//                 .fg(Color::White)
//                 .bg(Color::Red)
//                 .add_modifier(Modifier::BOLD),
//         ))
//         .title_alignment(Alignment::Right);
//     f.render_widget(block, top_chunks[1]);

//     // Bottom two inner blocks
//     let bottom_chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
//         .split(chunks[1]);

//     // Bottom left block with all default borders
//     let block = Block::default().title("With borders").borders(Borders::ALL);
//     f.render_widget(block, bottom_chunks[0]);

//     // Bottom right block with styled left and right border
//     let block = Block::default()
//         .title("With styled borders and doubled borders")
//         .border_style(Style::default().fg(Color::Cyan))
//         .borders(Borders::LEFT | Borders::RIGHT)
//         .border_type(BorderType::Double);
//     f.render_widget(block, bottom_chunks[1]);
// }
