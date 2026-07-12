mod app;
mod core;

fn main() {
    if let Err(why) = app::run_app() {
        eprintln!("{why}");
    }
}

