#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use raekna_ui::show_ui;

mod calculator;

fn main() {
    let calculator = Box::new(calculator::Calculator::default());
    show_ui(calculator);
}
