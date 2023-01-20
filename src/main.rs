mod ui;
mod app;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ui::UI;
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend}, Terminal,
};



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
    // let menu: Menu = Menu::new();
    let ui = UI::new();
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

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App, mut ui: UI) -> io::Result<()> {
    loop {
        // Draw the ui defined in the ui function with the data contained in App.
        // TODO: ui should maybe be a struct, instead of just a method? It's getting a tad complicated.
        // TODO: Why is app mutable when passing it to draw? It should be mutable when handling input, but not
        //   when drawing?

        // TODO: Pass terminal to UI on struct creation, so you can call draw in there. Have it accept a backend, so all rendering code is hidden in UI.
        let res = terminal.draw(|f| ui.render(f, app.help_visible));
        // ui.render(f, &app);
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => ui.select_next_menu_entry(),
                KeyCode::Up => ui.select_prev_menu_entry(),
                KeyCode::Char('h') => app.toggle_help(),
                _ => {}
            }

            match res {
                Ok(_) => continue,
                Err(_) => return Result::Err(res.err().unwrap()),
            }
        }
    }
}