mod app;
mod core;
mod views;

fn main() {
    if let Err(why) = app::run_app() {
        eprintln!("{why}");
    }
}
