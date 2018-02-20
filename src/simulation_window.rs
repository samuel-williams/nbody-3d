/* Simulation window. */

use glium;
// use input::*;
use render::{Renderer, RenderState};
use shaders;
use simulation::{Orbit, Simulation};
use teapot;
use view_controller::ViewController;
use std::{iter, time, thread, vec};

#[derive(Copy, Clone)]
pub enum Template {
    Binary,
    Unary,
    Cloud,
}

pub struct SimulationWindow<'a> {
    events_loop: glium::glutin::EventsLoop,
    // display: glium::Display,
    renderer: Renderer<'a>,
    simulation: Simulation, 
    view_controller: ViewController,
}

impl<'a> SimulationWindow<'a> {
    pub fn new(template: Template) -> Self {
        let mut events_loop = glium::glutin::EventsLoop::new();
        let display = SimulationWindow::setup_window(&events_loop);
        let renderer = Renderer::new(display);

        let mut simulation = match template {
            Template::Binary => Simulation::new_binary(4.0, 5.0),
            Template::Unary => Simulation::new_unary(),
            Template::Cloud => unimplemented!(),
        };

        // for _ in 1..20 {
        //     simulation.add_rand();
        // }

        SimulationWindow {
            events_loop: events_loop,
            // display: display,
            renderer: renderer,
            simulation: simulation,
            view_controller: ViewController::new(),
        }
    }

    fn setup_window(events_loop: &glium::glutin::EventsLoop) -> glium::Display {
        let window = glium::glutin::WindowBuilder::new()
            .with_dimensions(1280, 720)
            .with_title("nbody-3D simulation");
        let context = glium::glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_multisampling(16);

        glium::Display::new(
            window,
            context,
            events_loop
        ).unwrap()
    }

    
    pub fn start(&mut self) {
        let frame_budget = time::Duration::from_millis(16);

        let mut last_report_time = time::Instant::now();
        let mut total_times = vec::Vec::new();
        let mut draw_times = vec::Vec::new();
        let mut update_times = vec::Vec::new();

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
            // self.handle_menu_messages();
            self.view_controller.handle_events(&mut self.events_loop);

            /* TODO: fix these */
            if self.view_controller.input.clear {
                self.view_controller.input.clear = false;
                self.simulation.clear_paths();
            }
            if self.view_controller.input.add {
                self.view_controller.input.add = false;
                self.simulation.add_rand(Orbit::GreatestForce, 1.0);
            }

            if self.view_controller.running {            
                self.simulation.tick();
                // self.simulation.tick(); // 2 ticks per frame
            }
            let update_time = now.elapsed();

            let total_frame_time = draw_time + update_time;
            if total_frame_time < frame_budget {
                thread::sleep(frame_budget - total_frame_time);
            }

            /* Print statistics. */
            draw_times.push(draw_time);
            update_times.push(update_time);
            total_times.push(draw_time + update_time);
            if last_report_time.elapsed() > time::Duration::from_secs(5) {
                let draw_sum: time::Duration = draw_times.iter().sum();
                let update_sum: time::Duration = update_times.iter().sum(); 
                let total_sum: time::Duration = total_times.iter().sum();

                let draw_avg: time::Duration = draw_sum / draw_times.len() as u32;
                let update_avg: time::Duration = update_sum / update_times.len() as u32;
                let total_avg: time::Duration = total_sum / total_times.len() as u32;

                let draw_worst: time::Duration = *draw_times.iter().max().unwrap();
                let update_worst: time::Duration = *update_times.iter().max().unwrap();
                let total_worst: time::Duration = *total_times.iter().max().unwrap();

                println!("statistics for system with {} objects", self.simulation.teapots().len());
                println!("  avg times: ");
                println!("    draw: {:.2} ms", draw_avg.as_secs() as f64 + draw_avg.subsec_nanos() as f64 * 1e-6); 
                println!("    update: {:.2} ms", update_avg.as_secs() as f64 + update_avg.subsec_nanos() as f64 * 1e-6);
                println!("    total: {:.2} ms", total_avg.as_secs() as f64 + total_avg.subsec_nanos() as f64 * 1e-6);

                println!("  worst times: ");
                println!("    draw: {:.2} ms", draw_worst.as_secs() as f64 + draw_worst.subsec_nanos() as f64 * 1e-6); 
                println!("    update: {:.2} ms", update_worst.as_secs() as f64 + update_worst.subsec_nanos() as f64 * 1e-6);
                println!("    total: {:.2} ms\n", total_worst.as_secs() as f64 + total_worst.subsec_nanos() as f64 * 1e-6);
                
                /* Reset. */
                last_report_time = time::Instant::now();
                draw_times.clear();
                update_times.clear();
                total_times.clear();
            }
        }
    }

    // fn handle_menu_messages(&mut self) {
    //     let messages = self.menu.get_messages();

    //     let input = &mut self.view_controller.input;

    //     for message in messages {
    //         match message {
    //             MenuMsg::ToggleRun => {
    //                 input.running = !input.running;
    //                 println!("ToggleRun");
    //             },
    //             MenuMsg::ToggleTrails => input.trails = !input.trails,
    //             MenuMsg::Close => input.close = true,
    //             MenuMsg::Clear => self.simulation.clear_paths(),
    //         }
    //     }
    // }
}