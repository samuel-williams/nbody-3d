/* Encapsulate app context and event handling. */

use cgmath::{Matrix4, Rad, Vector3};
use glium::glutin::EventsLoop;
use input::{Input, Pan, Tilt};
use scene::Scene;

pub struct Context {
    input:  Input,
    scene:  Scene
}

const D_PAN: Rad<f32> = Rad(0.01);
const D_TILT: Rad<f32> = Rad(0.01);   

impl Context {
    pub fn new() -> Context {
        Context {
            input:  Input::new(),

            /* Construct scene. */
            scene:  Scene {
                pos:    Vector3 { x: 0.0, y: 0.0, z: -5.0 },
                ..      Scene::new()
            }
        }
    }

    pub fn handle_events(&mut self, events_loop: &mut EventsLoop) 
                         -> (bool, bool) {
        /* Update input struct. */
        self.input.handle_events(events_loop);

        /* Translate camera according to input. */
        self.scene.translate(self.input.mvmt, self.input.strafe);

        /* Pan camera according to input. */
        if let Some(pan) = self.input.pan.as_ref() {
            match pan {
                &Pan::Left => self.scene.pan(-D_PAN),
                &Pan::Right => self.scene.pan(D_PAN)
            }
        }

        /* Tilt camera according to input. */
        if let Some(tilt) = self.input.tilt.as_ref() {
            match tilt {
                &Tilt::Down => self.scene.tilt(-D_TILT),
                &Tilt::Up => self.scene.tilt(D_TILT)
            }
        }

        self.scene.update_view();

        (self.input.close, self.input.running)
    }

    pub fn view_matrix(&mut self) -> &Matrix4<f32> {
        &self.scene.view
    }
}