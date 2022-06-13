use raekna_common::RCalculator;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{constants::CARET_PERIOD, coordinator::Coordinator};

pub fn run_event_loop(calculator: Box<dyn RCalculator>) {
    let event_loop = EventLoop::new();
    let mut coordinator = Coordinator::new(&event_loop, calculator);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == coordinator.window.id() => {
                if !coordinator.handle_window_event(event) {
                    if let WindowEvent::CloseRequested = event {
                        *control_flow = ControlFlow::Exit
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == coordinator.window.id() => {
                match coordinator.redraw() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => coordinator.reconfigure_surface(),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                *control_flow = match coordinator.on_main_events_cleared() {
                    Some(last_update) => {
                        let next_update = last_update
                            .checked_add(std::time::Duration::from_millis(CARET_PERIOD as u64))
                            .unwrap();
                        ControlFlow::WaitUntil(next_update)
                    }
                    None => ControlFlow::Wait,
                };
            }
            _ => {}
        }
    });
}
