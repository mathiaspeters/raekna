#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use raekna_ui::run_app;

mod calculator;

fn main() -> Result<(), impl std::error::Error> {
    let calculator = Box::new(calculator::Calculator::default());
    run_app(calculator)
}
