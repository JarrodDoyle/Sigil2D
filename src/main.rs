use anyhow::Result;
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

pub fn main() -> Result<()> {
    env_logger::init();

    let (event_loop, window) = make_window()?;
    run(event_loop, window)?;

    Ok(())
}

pub fn run(event_loop: EventLoop<()>, window: Window) -> Result<()> {
    event_loop.run(|event, elwt| match event {
        Event::WindowEvent { window_id, event } if window_id == window.id() => {
            handle_window_event(event, elwt);
        }
        _ => (),
    })?;

    Ok(())
}

fn make_window() -> Result<(EventLoop<()>, Window)> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let window = WindowBuilder::new()
        .with_title("Sigil")
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)?;

    Ok((event_loop, window))
}

fn handle_window_event(event: WindowEvent, elwt: &EventLoopWindowTarget<()>) {
    if let WindowEvent::CloseRequested = event {
        elwt.exit();
    }
}
