
use futures::executor::block_on;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera;
mod model;
mod render;
mod simulation;
mod texture;

fn window_to_view_space(window_size: PhysicalSize<u32>, window_position: PhysicalPosition<f64>) -> cgmath::Vector2<f64> {
    cgmath::Vector2 {
        x: window_position.x / window_size.width as f64,
        y: window_position.y / window_size.height as f64,
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut simulation = simulation::Simulation::new();
    let mut running = true;
    let mut last_cursor_position: Option<PhysicalPosition<f64>> = None;
    let mut shift_down = false;
    let mut ctrl_down = false;
    
    let mut render_state = block_on(render::State::new(&window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !render_state.input(event) {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    KeyboardInput { 
                        state: ElementState::Pressed, 
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    } => running = !running,
                    KeyboardInput { 
                        state, 
                        virtual_keycode: Some(VirtualKeyCode::LShift),
                        ..
                    } => shift_down = *state == ElementState::Pressed,
                    KeyboardInput { 
                        state, 
                        virtual_keycode: Some(VirtualKeyCode::LControl),
                        ..
                    } => ctrl_down = *state == ElementState::Pressed,
                    _ => {}
                }

                WindowEvent::CursorMoved { position, .. } => {
                    last_cursor_position = Some(*position);
                }

                WindowEvent::CursorLeft { .. } => {
                    last_cursor_position = None;
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    if *state == ElementState::Released && *button == MouseButton::Left {
                        if let Some(position) = last_cursor_position {
                            let view_position = window_to_view_space(window.inner_size(), position);
                            let barycentric_position = render_state.view_to_world_space(view_position);

                            let mass = if ctrl_down { 
                                simulation::BodyMass::Large 
                            } else if shift_down {
                                simulation::BodyMass::Medium
                            } else {
                                simulation::BodyMass::Small
                            };

                            simulation.add_body_at_position(barycentric_position, mass);
                        }
                    }
                }

                WindowEvent::Resized(physical_size) => {
                    render_state.resize(Some(*physical_size));
                }

                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    render_state.resize(Some(**new_inner_size));
                }
                _ => {}
            } 
        }

        Event::RedrawRequested(_) => {
            if running {
                simulation.tick();
            }
                
            let barycenter = simulation.barycenter();
            render_state.update_light((barycenter.x as f32, barycenter.y as f32, barycenter.z as f32).into());
            render_state.update_camera((barycenter.x as f32, barycenter.y as f32, barycenter.z as f32).into());

            let instances = simulation.instances();
            render_state.update_instances(instances);

            match render_state.render() {
                Ok(_) => {}
                Err(wgpu::SwapChainError::Lost) => render_state.resize(None),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }

        Event::MainEventsCleared => {
            window.request_redraw()
        }

        _ => {}
    });
}
