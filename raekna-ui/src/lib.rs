use raekna_common::RCalculator;

mod constants;
mod coordinator;
mod event_loop;
mod graphics;

pub fn show_ui(calculator: Box<dyn RCalculator>) {
    env_logger::init();
    event_loop::run_event_loop(calculator);
}
