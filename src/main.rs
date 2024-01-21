mod world;
use std::time::{Duration, Instant};

use world::World;

use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new((WIDTH / 4, HEIGHT / 4), 4);

    let mut paused = false;
    let mut start_time = Instant::now();
    let mut frames = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                paused = !paused;
            }

            if input.mouse_held(0) {
                if let Some((x, y)) = input.mouse() {
                    world.setat((x as u32, y as u32), world::Particle::Dirt);
                }
            }

            // if let Some(size) = input.window_resized() {
            //     if let Err(err) = pixels.resize_surface(size.width, size.height) {
            //         log_error("pixels.resize_surface", err);
            //         *control_flow = ControlFlow::Exit;
            //         return;
            //     }
            // }

            if !paused {
                world.update();
            }
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
            world.draw(pixels.frame_mut());
            frames += 1;
        }

        let now = Instant::now();
        let diff = now.duration_since(start_time);
        if diff > Duration::from_secs(1) {
            println!("fps: {}", frames as f64 / diff.as_secs_f64());
            start_time = now;
            frames = 0;
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
