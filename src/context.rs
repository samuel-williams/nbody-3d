/* Encapsulate app context and event handling. */

use cgmath::{Matrix4};
use glium::glutin::EventsLoop;
use input::{Input};
use scene::Scene;

pub struct Context {
    input:  Input,
    scene:  Scene
}  

impl Context {
    pub fn new() -> Context {
        Context {
            input:  Input::new(),

            /* Construct scene. */
            scene:  Scene::new()
        }
    }

    pub fn handle_events(&mut self, events_loop: &mut EventsLoop) 
                         -> (bool, bool, bool) {
        /* Update input struct. */
        self.input.handle_events(events_loop);

        /* Update scene for input. */
        self.scene.handle_input(&self.input);

        self.scene.update_view();

        (self.input.close, self.input.running, self.input.trails)
    }

    pub fn view_matrix(&mut self) -> &Matrix4<f32> {
        &self.scene.view
    }
}