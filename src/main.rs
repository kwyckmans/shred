
// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: std::path::PathBuf,
// }


use std::{io, thread, time::Duration};
use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{
    backend::{CrosstermBackend, Backend}, Terminal, widgets::{Block, Borders}, Frame, layout::{Direction, Layout, Constraint},
};

fn main() -> Result<(), io::Error> {
    // Disables a bunch of options in the terminal this app runs in, so you can do more stuff with it.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // Crossterm supports 2 screens, the main screen and the alternate screen.
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(ui)?;

    thread::sleep(Duration::from_millis(5000));

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

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
         .direction(Direction::Vertical)
         .margin(1)
         .constraints(
             [
                 Constraint::Percentage(10),
                 Constraint::Percentage(80),
                 Constraint::Percentage(10)
             ].as_ref()
         )
         .split(f.size());
     let block = Block::default()
          .title("Block")
          .borders(Borders::ALL);
     f.render_widget(block, chunks[0]);
     let block = Block::default()
          .title("Block 2")
          .borders(Borders::ALL);
     f.render_widget(block, chunks[1]);
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

// fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
//     loop {
//         terminal.draw(ui)?;

//         if let Event::Key(key) = event::read()? {
//             if let KeyCode::Char('q') = key.code {
//                 return Ok(());
//             }
//         }
//     }
// }

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
