/* Controller for simulation. */

use config::Config;
use render::*;
use simulation::*;
use view::*;

use glium::glutin::{
    self, 
    ElementState,
    EventsLoop,
    KeyboardInput, 
    VirtualKeyCode
};
use std::{thread, vec};
use time::{Duration, PreciseTime};

pub struct Controller<'a> {
    simulation: &'a mut Simulation<'a>,
    view: View,
    run_sim: bool,
    run_app: bool,
    trails: bool,
}

impl<'a> Controller<'a> {
    pub fn new(config: Config, simulation: &'a mut Simulation<'a>) -> Self {
        Controller {
            simulation: simulation,
            view: View::new(),
            run_sim: false,
            run_app: true,
            trails: true,
        }
    }

    pub fn start_simulation(&mut self, events_loop: &mut EventsLoop) {
        let frame_budget = Duration::milliseconds(16);

        let mut last_report_time = PreciseTime::now();
        let mut total_times = vec::Vec::new();
        let mut draw_times = vec::Vec::new();
        let mut update_times = vec::Vec::new();

        let mut last_frame_time = Duration::zero();

        while self.run_app {
            let frame_begin_time = PreciseTime::now();
            
            self.handle_events(last_frame_time, events_loop);

            if self.run_sim {            
                self.simulation.tick();
                // self.simulation.tick(); // 2 ticks per frame
            }
            let update_time = frame_begin_time.to(PreciseTime::now());

            let draw_time = self.simulation.renderer().draw(
                    RenderState { 
                        draw_trails: self.trails,
                        light_pos: [-3.0, 1.5, 3.0f32],
                        view_matrix: self.view.view_matrix(), 
                    }, 
                    self.simulation.teapots(),
                );            

            let total_time = draw_time + update_time;
            if total_time < frame_budget {
                thread::sleep(frame_budget.to_std().unwrap() - total_time.to_std().unwrap());
            }

            /* Print statistics. */
            draw_times.push(draw_time);
            update_times.push(update_time);
            total_times.push(draw_time + update_time);
            if last_report_time.to(PreciseTime::now()) > Duration::seconds(5) {
                let draw_sum: Duration = draw_times
                        .iter()
                        .fold(Duration::zero(), |sum, &val| { sum + val });

                let update_sum: Duration = update_times
                        .iter()
                        .fold(Duration::zero(), |sum, &val| { sum + val });
                
                let total_sum: Duration = total_times
                        .iter()
                        .fold(Duration::zero(), |sum, &val| { sum + val });

                let draw_avg: Duration = draw_sum / draw_times.len() as i32;
                let update_avg: Duration = update_sum / update_times.len() as i32;
                let total_avg: Duration = total_sum / total_times.len() as i32;

                let draw_worst: Duration = *draw_times.iter().max().unwrap();
                let update_worst: Duration = *update_times.iter().max().unwrap();
                let total_worst: Duration = *total_times.iter().max().unwrap();

                println!("statistics for system with {} objects", self.simulation.teapots().len());
                println!("  avg times: ");
                println!("    draw: {} ms", draw_avg.num_milliseconds()); 
                println!("    update: {:.2} ms", update_avg.num_milliseconds());
                println!("    total: {:.2} ms", total_avg.num_milliseconds());

                println!("  worst times: ");
                println!("    draw: {:.2} ms", draw_worst.num_milliseconds()); 
                println!("    update: {:.2} ms", update_worst.num_milliseconds());
                println!("    total: {:.2} ms\n", total_worst.num_milliseconds());
                
                /* Reset. */
                last_report_time = PreciseTime::now();
                draw_times.clear();
                update_times.clear();
                total_times.clear();
            }

            last_frame_time = frame_begin_time.to(PreciseTime::now());
        }
    }

    fn handle_events(&mut self, dt: Duration, events_loop: &mut EventsLoop) {
        let dt_float = dt.num_microseconds().unwrap() as f32 * 1e-6;
        // let events_loop = self.simulation.events_loop();

        events_loop.poll_events(|ev| { 
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => self.run_app = false,
                    glutin::WindowEvent::KeyboardInput { input, .. } => 
                        self.handle_key(input, dt_float),
                    _ => (),
                },
                _ => (),
            }
        });

        self.view.update_view();
    }

    fn handle_key(&mut self, input: glutin::KeyboardInput, dt: f32) {
        /* Keymap. */

        match input.virtual_keycode {
            Some(VirtualKeyCode::Up) => {
                if input.state == ElementState::Pressed {
                    self.view.tilt(Tilt::Up, dt)
                }
            },

            Some(VirtualKeyCode::Down) => {
                if input.state == ElementState::Pressed {
                    self.view.tilt(Tilt::Down, dt)
                }
            },

            Some(VirtualKeyCode::Left) => {
                if input.state == ElementState::Pressed {
                    self.view.pan(Pan::Left, dt)
                }
            },

            Some(VirtualKeyCode::Right) => {
                if input.state == ElementState::Pressed {
                    self.view.pan(Pan::Right, dt)
                }
            },

            Some(VirtualKeyCode::W) => {
                if input.state == ElementState::Pressed {
                    self.view.movez(MoveZ::Neg, dt)
                }
            },

            Some(VirtualKeyCode::S) => {
                if input.state == ElementState::Pressed {
                    self.view.movez(MoveZ::Pos, dt)
                }
            },

            Some(VirtualKeyCode::A) => {
                if input.state == ElementState::Pressed {
                    self.view.movex(MoveX::Neg, dt)
                }
            },

            Some(VirtualKeyCode::D) => {
                if input.state == ElementState::Pressed {
                    self.view.movex(MoveX::Pos, dt)
                }
            },

            Some(VirtualKeyCode::Z) => {
                if input.state == ElementState::Pressed {
                    self.view.zoom(Zoom::In, dt)
                }
            },

            Some(VirtualKeyCode::X) => {
                if input.state == ElementState::Pressed {
                    self.view.zoom(Zoom::Out, dt)
                }
            },

            Some(VirtualKeyCode::Space) => { 
                if input.state == ElementState::Released {
                    self.run_sim = !self.run_sim
                }
            },

            Some(VirtualKeyCode::P) => {
                if input.state == ElementState::Released {
                    self.trails = !self.trails
                }
            },

            Some(VirtualKeyCode::C) => {
                if input.state == ElementState::Released {
                    self.simulation.clear_paths()
                }
            },

            Some(VirtualKeyCode::R) => {
                if input.state == ElementState::Released {
                    self.simulation.add_rand(Orbit::GreatestForce, 1.0)
                }
            },

            // Some(VirtualKeyCode::M) => {
            //     use menu;
            //     menu::open();
            //     self.menu = true;
            // },

            _ => (),
        }
    }

}