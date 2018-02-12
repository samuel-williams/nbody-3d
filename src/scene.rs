/* Manipulate scene. */

extern crate cgmath;
extern crate glium;
extern crate rand;

use cgmath::{InnerSpace, Matrix4, Rad, vec3, Vector3};
use input::{Input, MoveX, MoveZ, Pan, Tilt, Zoom};
use math::clamp;
use std::f32::consts::{PI, FRAC_PI_2};


const D_POS: f32 = 0.05;
const D_PAN: Rad<f32> = Rad(0.01);
const D_TILT: Rad<f32> = Rad(0.01); 
const R_DIST: f32 = 1.05;

/* Default values for Scene members. */
const DEF_POS:  Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0};
const DEF_PAN:  Rad<f32> = Rad(0.0);
const DEF_TILT: Rad<f32> = Rad(0.0);
const DEF_DIST: f32 = 3.0;

/* Hold camera parameters. */
pub struct Scene {
    pub pos:    Vector3<f32>,
    pub pan:    Rad<f32>,
    pub tilt:   Rad<f32>,
    pub dist:   f32,
    pub view:   Matrix4<f32>, // dont access this directly
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            pos:    DEF_POS,
            pan:    DEF_PAN,
            tilt:   DEF_TILT,
            dist:   DEF_DIST,
            view:   Scene::construct_view(DEF_POS, DEF_PAN, DEF_TILT, DEF_DIST)
        }
    }

    pub fn handle_input(&mut self, input: &Input) {
        /* Handle zoom. */
        if let Some(zoom) = input.zoom {
            self.dist = match zoom {
                Zoom::In => self.dist * R_DIST,
                Zoom::Out => self.dist / R_DIST
            }
        }

        /* Handle pan. */
        if let Some(pan) = input.pan {
            self.pan += match pan {
                Pan::Left => -D_PAN,
                Pan::Right => D_PAN
            };
            self.pan %= Rad(2.0 * PI)
        }

        /* Handle tilt. */
        if let Some(tilt) = input.tilt {
            self.tilt += match tilt {
                Tilt::Down => -D_TILT,
                Tilt::Up => D_TILT
            };
            self.tilt = clamp(self.tilt, 
                              Rad(-FRAC_PI_2 + 0.0001), 
                              Rad(FRAC_PI_2 - 0.0001))
        }

        /* Handle move focus point X. */
        if let Some(move_x) = input.move_x {
            let x_dir = vec3(self.view.x[0], 0.0, self.view.z[0]).normalize();
            self.pos += x_dir * match move_x {
                MoveX::Pos => D_POS,
                MoveX::Neg => -D_POS,
            }
        }

        /* Handle move focus point Z. */
        if let Some(move_z) = input.move_z {
            let z_dir = vec3(self.view.x[2], 0.0, self.view.z[2]).normalize();
            self.pos += z_dir * match move_z {
                MoveZ::Pos => D_POS,
                MoveZ::Neg => -D_POS,
            }
        }

        if let Some(mouse_press) = input.mouse_press {
            println!("Mouse pressed at {}, {}.", mouse_press.pos.0, mouse_press.pos.1)
        }

    }

    /* Update view matrix. */
    pub fn update_view(&mut self) {
        self.view = Scene::construct_view(self.pos, self.pan, self.tilt, self.dist)
    }

    /* Construct view matrix (helper for update_view). */
    fn construct_view(pos: Vector3<f32>, pan: Rad<f32>, 
                      tilt: Rad<f32>, dist: f32) -> Matrix4<f32> {
        let zoom = Matrix4::from_translation(vec3(0.0, 0.0, -dist));
        let trans_focus = Matrix4::from_translation(-pos);
        let xrot = Matrix4::from_angle_x(tilt);
        let yrot = Matrix4::from_angle_y(pan);
        zoom * xrot * yrot * trans_focus
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