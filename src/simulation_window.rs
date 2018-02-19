/* Simulation window. */

use glium;
// use input::*;
use menu::{MenuHandle, MenuMsg};
use render::{Renderer, RenderState};
use shaders;
use simulation::Simulation;
use teapot;
use view_controller::ViewController;
use std::{time, thread};

pub struct SimulationWindow<'a> {
    events_loop: glium::glutin::EventsLoop,
    // display: glium::Display,
    renderer: Renderer<'a>,
    simulation: Simulation, 
    view_controller: ViewController,
    menu: MenuHandle,
}

impl<'a> SimulationWindow<'a> {
    pub fn new(menu: MenuHandle) -> Self {
        let mut events_loop = glium::glutin::EventsLoop::new();
        let display = SimulationWindow::setup_window(&events_loop);
        let renderer = Renderer::new(display);
        let mut simulation = Simulation::new_binary(4.0, 5.0);

        // for _ in 1..20 {
        //     simulation.add_rand();
        // }

        SimulationWindow {
            events_loop: events_loop,
            // display: display,
            renderer: renderer,
            simulation: simulation,
            view_controller: ViewController::new(),
            menu: menu,
        }
    }

    fn setup_window(events_loop: &glium::glutin::EventsLoop) -> glium::Display {
        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(1280, 720)
            .with_title("Glium");
        let context = glium::glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_multisampling(0);
        
        glium::Display::new(
            window,
            context,
            events_loop
        ).unwrap()
    }

    
    pub fn start(&mut self) {
        let frame_budget = time::Duration::from_millis(32);
        let mut running = true;
        while !self.view_controller.close {
            let draw_time = self.renderer.draw(
                    RenderState { 
                        draw_trails: self.view_controller.trails,
                        light_pos: [-3.0, 1.5, 3.0f32],
                        view_matrix: self.view_controller.view_matrix(), 
                    }, 
                    self.simulation.teapots(),
                );

            let now = time::Instant::now();
            self.handle_menu_messages();
            self.view_controller.handle_events(&mut self.events_loop);

            /* TODO: fix these */
            if self.view_controller.input.clear {
                self.view_controller.input.clear = false;
                self.simulation.clear_paths();
            }
            if self.view_controller.input.add {
                self.view_controller.input.add = false;
                self.simulation.add_rand();
            }

            if self.view_controller.running {            
                self.simulation.tick();
                self.simulation.tick(); // 2 ticks per frame
            }
            let update_time = now.elapsed();

            if draw_time + update_time < frame_budget {
                thread::sleep(frame_budget - (draw_time + update_time));
            } else {
                println!(
                    "draw time: {}, update time: {}", 
                    draw_time.as_secs() as f64 + draw_time.subsec_nanos() as f64 * 1e-9,
                    update_time.as_secs() as f64 + update_time.subsec_nanos() as f64 * 1e-9
                );
            }
        }
    }

    fn handle_menu_messages(&mut self) {
        let messages = self.menu.get_messages();

        let input = &mut self.view_controller.input;

        for message in messages {
            match message {
                MenuMsg::ToggleRun => {
                    input.running = !input.running;
                    println!("ToggleRun");
                },
                MenuMsg::ToggleTrails => input.trails = !input.trails,
                MenuMsg::Close => input.close = true,
                MenuMsg::Clear => self.simulation.clear_paths(),
            }
        }
    }
}