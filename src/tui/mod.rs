use std::{
    error::Error,
    io::{self},
    path::PathBuf,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use crate::{
    app::{App, ClosingAction},
    repo::Tracking,
};
mod ui;

/// Step off point for the tui app
pub fn run() -> Result<(), Box<dyn Error>> {
    let mut tracking = Tracking::new();

    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let mut app = App::new(tracking);
    let res = run_app(&mut terminal, &mut app, tick_rate);

    tear_down(terminal, &res);

    match app.action {
        ClosingAction::Exit => {}
        ClosingAction::Cd => app.goto_dir(),
        ClosingAction::Editor => app.open_in_editor(),
    }

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn tear_down(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
    res: &io::Result<()>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

// Runs the main app loop here it should handle drawing the UI and event polling.
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.on_key(KeyCode::Char('q')),
                        KeyCode::Char('e') => app.on_key(KeyCode::Char('e')),
                        KeyCode::Enter => app.on_key(KeyCode::Enter),
                        KeyCode::Up => app.on_up(),
                        KeyCode::Down => app.on_down(),
                        _ => (),
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.quit {
            return Ok(());
        }
    }
}
