mod app;
mod core;
pub mod tui;
mod views;

fn main() {
    if let Err(why) = app::run_app() {
        eprintln!("{why}");
    }
}
