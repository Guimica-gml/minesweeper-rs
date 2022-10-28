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

fn main() {
    let mut args = env::args().skip(1);
    let mode = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("Error: Expected execution mode argument");
            process::exit(1);
        }
    };

    if mode == "--window" {
        match window::main() {
            Ok(_) => {},
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    else if mode == "--terminal" {
        terminal::main();
    }
    else {
        eprintln!("Error: invalid execution mode");
        usage();
    }
}
