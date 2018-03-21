extern crate cgmath;
#[macro_use] extern crate conrod;
#[macro_use] extern crate glium;
extern crate rand;
extern crate rayon;
extern crate time;

mod config;
mod controller;
// mod gl_vertex;
// mod input;
// mod math;
mod physics;
mod start_menu;
mod render;
// mod scene;
mod shaders;
mod simulation;
// mod simulation_window;
mod teapot;
mod teapot_obj;
mod view;

use controller::Controller;
use simulation::Simulation;

fn main() {
    let config = config::Config::from_file("config.cfg");
    let mut events_loop = glium::glutin::EventsLoop::new();

    /* Setup simulation. */
    let mut simulation;
    if let Some(template) = start_menu::StartMenu::new().execute() {
        simulation = Box::new(Simulation::from_template(template, config, &events_loop))
    } else {
        return
    }

    /* Setup controller. */
    let mut controller = Controller::new(config, &mut simulation);

    /* Run. */
    controller.start_simulation(&mut events_loop);
}