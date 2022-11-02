use std::{
    io,
    process,
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode
    },
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    },
    execute,
};

fn main() {
    if let Err(e) = enable_raw_mode() {
        eprintln!("enable_raw_mode error {e}");
        process::exit(1);
    };
    let mut stdout = io::stdout();
    if let Err(e) = execute!(stdout, EnterAlternateScreen, EnableMouseCapture) {
        eprintln!("execute error {e}");
        process::exit(1);
    };
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
    if let Err(e) = stockrs::run(&mut terminal) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    if let Err(e) = disable_raw_mode() {
        eprintln!("disable_raw_mode error {e}");
        process::exit(1);
    }
    if let Err(e) = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ) {
        eprintln!("execute error {e}");
        process::exit(1);
    };
    if let Err(e) = terminal.show_cursor() {
        eprintln!("terminal.show_cursor error {e}");
        process::exit(1);
    }
    
}

