use raekna_common::RCalculator;
use winit::event_loop::EventLoop;

use crate::coordinator::Coordinator;

mod constants;
mod coordinator;
mod graphics;

pub fn run_app(calculator: Box<dyn RCalculator>) -> Result<(), impl std::error::Error> {
    env_logger::init();
    let mut coordinator = Coordinator::new(calculator);
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut coordinator)
}
