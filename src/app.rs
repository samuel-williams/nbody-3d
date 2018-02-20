extern crate cgmath;
#[macro_use] extern crate conrod;
#[macro_use] extern crate glium;
extern crate rand;
extern crate rayon;

mod gl_vertex;
mod input;
mod math;
mod start_menu;
mod render;
mod scene;
mod shaders;
mod simulation;
mod simulation_window;
mod teapot;
mod teapot_obj;
mod view_controller;

// use menu::MenuHandle;
use simulation_window::SimulationWindow;

fn main() {
    App::new().start();
}

struct App<'a> {
    // menu: MenuHandle,
    sim_window: Option<SimulationWindow<'a>>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let template_selection = start_menu::StartMenu::new().execute();

        App {
            sim_window: 
                if let Some(template) = template_selection {
                    Some(SimulationWindow::new(template))
                } else { None },
        }
    }

    pub fn start(&mut self) {
        if let Some(ref mut sim_window) = self.sim_window {
            sim_window.start();
        }
    }
}