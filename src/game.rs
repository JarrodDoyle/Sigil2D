use std::sync::Arc;

use anyhow::Result;
use rune::{alloc::clone::TryClone, Vm};
use wgpu::Limits;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::WindowBuilder,
};

use crate::{gfx::Context, input::Input, scripting};

pub struct GameConfig {
    source_dir: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            source_dir: format!("{}/scripts", env!("CARGO_MANIFEST_DIR")),
        }
    }
}

pub struct Game<'w> {
    event_loop: EventLoop<()>,
    context: Context<'w>,
    vm: Vm,
    config: GameConfig,
}

impl<'w> Game<'w> {
    pub fn new(config: GameConfig) -> Result<Self> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);

        let window = WindowBuilder::new()
            .with_title("Sigil")
            .with_inner_size(LogicalSize::new(1280.0, 720.0))
            .build(&event_loop)?;

        let context = pollster::block_on(Context::new(Arc::new(window), Limits::default()))?;
        let runtime = scripting::Runtime::new(&["frame_counter"])?;

        Ok(Self {
            event_loop,
            context,
            vm: runtime.vm.try_clone()?,
            config,
        })
    }

    pub fn run(mut self) -> Result<()> {
        let frame_counter = self.vm.call(["FrameCounter", "new"], ())?;
        let mut input = Input::new();

        self.event_loop.run(|event, elwt| {
            match event {
                Event::WindowEvent { window_id, event }
                    if window_id == self.context.window.id() =>
                {
                    if self.context.handle_window_event(&event, elwt) {
                        return;
                    }

                    if let WindowEvent::RedrawRequested = event {
                        render(&self.context);
                        self.context.window.request_redraw();
                    }

                    input.update(&event);
                }
                _ => (),
            }

            if input.is_key_just_pressed(KeyCode::Escape) {
                elwt.exit();
            }

            self.vm
                .call(["FrameCounter", "update"], (&frame_counter,))
                .unwrap();
        })?;

        Ok(())
    }
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
