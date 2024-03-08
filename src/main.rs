mod gfx;
mod input;
mod scripting;

use std::sync::Arc;

use anyhow::Result;
use gfx::Context;
use input::Input;
use wgpu::Limits;
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowBuilder},
};

pub fn main() -> Result<()> {
    env_logger::init();

    let (event_loop, window) = make_window()?;
    let context = pollster::block_on(Context::new(Arc::new(window), Limits::default()))?;
    run(event_loop, context)?;

    Ok(())
}

pub fn run(event_loop: EventLoop<()>, mut context: Context) -> Result<()> {
    let mut runtime = scripting::Runtime::new(&["frame_counter"])?;

    let frame_counter = runtime.vm.call(["FrameCounter", "new"], ())?;
    let mut input = Input::new();

    event_loop.run(|event, elwt| {
        match event {
            Event::WindowEvent { window_id, event } if window_id == context.window.id() => {
                if context.handle_window_event(&event, elwt) {
                    return;
                }

                if let WindowEvent::RedrawRequested = event {
                    render(&context);
                    context.window.request_redraw();
                }

                input.update(&event);
            }
            _ => (),
        }

        if input.is_key_just_pressed(KeyCode::Escape) {
            elwt.exit();
        }

        runtime
            .vm
            .call(["FrameCounter", "update"], (&frame_counter,))
            .unwrap();
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

fn render(context: &Context) {
    let output = context.surface.get_current_texture().unwrap();
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = context
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
    {
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
    }

    // submit will accept anything that implements IntoIter
    context.queue.submit(std::iter::once(encoder.finish()));
    output.present();
}
