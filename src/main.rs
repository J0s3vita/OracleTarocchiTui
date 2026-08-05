//! Oracle Deck TUI — mazzo di tarocchi/oracoli cyberpunk da terminale.

mod app;
mod deck;
mod draw;
mod export;
mod model;
mod rng;
mod store;
mod theme;
mod ui;

use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyEventKind};

use crate::app::App;

/// ~30 fps: fluido per shuffle, glitch e particelle.
const FRAME: Duration = Duration::from_millis(33);

fn main() -> io::Result<()> {
    if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            "-V" | "--version" => {
                println!("oracledecktui {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "--path" => {
                println!("{}", store::data_dir().display());
                return Ok(());
            }
            other => {
                eprintln!("Argomento non riconosciuto: {other}\n");
                print_usage();
                std::process::exit(2);
            }
        }
    }

    if let Err(e) = store::ensure_dirs() {
        eprintln!("Impossibile preparare {}: {e}", store::data_dir().display());
        std::process::exit(1);
    }

    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn print_usage() {
    println!(
        "oracledecktui — mazzo di tarocchi/oracoli cyberpunk da terminale\n\n\
         USO:\n    oracledecktui\n\n\
         OPZIONI:\n    \
         -h, --help       questo messaggio\n    \
         -V, --version    versione\n    \
         --path           mostra la cartella dati\n\n\
         AMBIENTE:\n    \
         ORACLE_DIR       cartella dati alternativa (default ~/.oracledecktui)"
    );
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    while !app.should_quit {
        app.tick();
        terminal.draw(|frame| ui::render(frame, &app))?;

        let deadline = Instant::now() + FRAME;
        while let Some(remaining) = deadline.checked_duration_since(Instant::now()) {
            if !event::poll(remaining)? {
                break;
            }
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => app.on_key(key),
                Event::Resize(_, _) => break,
                _ => {}
            }
        }
    }
    Ok(())
}
