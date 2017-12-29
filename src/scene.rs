/* Manipulate scene. */

extern crate cgmath;
extern crate glium;
extern crate rand;

use cgmath::{InnerSpace, Matrix4, Rad, vec3, Vector3};
use input::{Movement, Strafe};
use math::clamp;
use std::f32::consts::{PI, FRAC_PI_2};

const D_POS: f32 = 0.05;

/* Hold camera parameters. */
pub struct Scene {
    pub pos:    Vector3<f32>,
    pub pan:    Rad<f32>,
    pub tilt:   Rad<f32>,
    pub view:   Matrix4<f32>, // dont access this directly
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            pos: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            pan: Rad(0.0),
            tilt: Rad(0.0),
            view: Matrix4::from_scale(1.0)
        }
    }

    /* Move the camera focus point. */
    pub fn translate(&mut self, mvmt: Option<Movement>,
                     strafe: Option<Strafe>) {
        let view = self.view;
        
        /* Handle move along camera z. */
        if let Some(mvmt) = mvmt {
            let dir = vec3(view.x[2], 0.0, view.z[2]).normalize();
            self.pos += D_POS * match mvmt {
                Movement::Backward => -dir,
                Movement::Forward => dir
            }
        }

        /* Handle strafe along camera x. */
        if let Some(strafe) = strafe {
            let right = vec3(view.x[0], view.y[0], view.z[0]).normalize();
            self.pos += D_POS * match strafe {
                Strafe::Left => right,
                Strafe::Right => -right
            }
        }
    }

    /* Pan the camera. */
    pub fn pan(&mut self, pan_amt: Rad<f32>) {
        self.pan = (self.pan + pan_amt) % Rad(2.0 * PI);
    }

    /* Tilt the camera. */
    pub fn tilt(&mut self, tilt_amt: Rad<f32>) {
        self.tilt = clamp(self.tilt + tilt_amt, 
                          Rad(-FRAC_PI_2 - 0.001), 
                          Rad(FRAC_PI_2 - 0.001));
    }

    /* Update view matrix. */
    pub fn update_view(&mut self) {
        self.view = Scene::construct_view(self.pos, self.pan, self.tilt)
    }

    /* Construct view matrix (helper for update_view). */
    fn construct_view(pos: Vector3<f32>, pan: Rad<f32>, 
                      tilt: Rad<f32>) -> Matrix4<f32> {
        let translate = Matrix4::from_translation(pos);
        let xrot = Matrix4::from_angle_x(tilt);
        let yrot = Matrix4::from_angle_y(pan);
        translate * xrot * yrot
    }

    // /* Get the view matrix. */
    // pub fn view_matrix(&mut self) -> &Matrix4<f32> {
    //     if let Some(view) = self.view_matrix {
    //         &view
    //     } else {
    //         self.update_view();
    //         &self.view_matrix.unwrap()
    //     }
    // }
}