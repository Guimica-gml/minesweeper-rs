use std::env;
use std::process;

mod mine;
mod window;
mod terminal;

fn usage() {
    eprintln!("Usage: program [--mode]");
    eprintln!("Execution modes:");
    eprintln!("    window: Runs the game in a sdl2 window");
    eprintln!("    terminal: Runs the game in a terminal");
}

fn main() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mode = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("Error: Expected execution mode argument");
            usage();
            process::exit(1);
        }
    };

    if mode == "--window" {
        window::main()?;
    }
    else if mode == "--terminal" {
        terminal::main().map_err(|e| e.to_string())?;
    }
    else {
        eprintln!("Error: invalid execution mode");
        usage();
        process::exit(1);
    }

    Ok(())
}
