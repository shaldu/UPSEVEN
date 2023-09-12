use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod game;
use game::Game;

use std::time::Instant;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    const TITLE: &str = "Cool Game";
    const FPS: u32 = 60;
    const RENDER_FPS: u32 = 120;

    let mut last_time = Instant::now();
    let event_loop = EventLoop::new();
    let mut game = Game::new();

    // Define the window using WindowBuilder and build it.
    let window = WindowBuilder::new()
        .with_title(TITLE) // Set window title
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT)) // Set window size
        .build(&event_loop)
        .unwrap();

    // Run the event loop.
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; // Wait for events rather than continuously looping

        let now = Instant::now();
        let delta_time = now.duration_since(last_time);
        last_time = now;

        //FPS = 60 
        if delta_time.as_secs_f32() > 1.0 / FPS as f32 {
            game.update(delta_time.as_secs_f32());
        }

        if delta_time.as_secs_f32() > 1.0 / RENDER_FPS as f32 {
            window.request_redraw();
        }

        match event {
            Event::RedrawRequested(_) => {
                game.render(delta_time.as_secs_f32());
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit; // Exit the event loop when the window is requested to close
                }
                _ => {}
            },
            _ => {}
        }

    });
}
