use std::process;

fn main() {
    if let Err(e) = stockrs::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

