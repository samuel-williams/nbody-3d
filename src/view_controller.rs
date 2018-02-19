/* Encapsulate app context and event handling. */

use cgmath::{Matrix4};
use glium::glutin::EventsLoop;
use input::{Input};
use menu::MenuMsg;
use scene::Scene;
use std::vec::Vec;

pub struct ViewController {
    pub input: Input,
    scene: Scene,
    pub running: bool,
    pub close: bool,
    pub trails: bool,
}  

impl ViewController {
    pub fn new() -> ViewController {
        ViewController {
            input: Input::new(),
            scene: Scene::new(),
            running: false,
            close: false,
            trails: true,
        }
    }

    pub fn handle_events(&mut self, events_loop: &mut EventsLoop) {
        /* Update input struct. */
        self.input.handle_events(events_loop);

        /* Update scene for input. */
        self.scene.handle_input(&self.input);

        self.scene.update_view();

        self.running = self.input.running;
        self.close = self.input.close;
        self.trails = self.input.trails;
    }

    pub fn view_matrix(&mut self) -> &Matrix4<f32> {
        &self.scene.view
    }
}