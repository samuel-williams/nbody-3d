extern crate cgmath;
#[macro_use] extern crate conrod;
#[macro_use] extern crate glium;
extern crate rand;

mod gl_vertex;
mod input;
mod math;
mod menu;
mod render;
mod scene;
mod shaders;
mod simulation;
mod simulation_window;
mod teapot;
mod teapot_obj;
mod view_controller;

use menu::MenuHandle;
use simulation_window::SimulationWindow;

fn main() {
    let mut app = App::new();
    app.start();
}

struct App<'a> {
    // menu: MenuHandle,
    sim_window: SimulationWindow<'a>,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let menu = MenuHandle::build();
        let sim_window = SimulationWindow::new(menu);
        App {
            // menu: menu,
            sim_window: sim_window,
        }
    }

    pub fn start(&mut self) {
        self.sim_window.start();
    }
}