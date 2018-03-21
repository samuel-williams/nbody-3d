/* View of simulation. */

extern crate cgmath;
extern crate glium;
extern crate rand;

use cgmath::{InnerSpace, Matrix4, Rad, vec3, Vector3};
use std::f32::consts::{PI, FRAC_PI_2};

#[derive(Copy, Clone)]
pub enum Pan { Right, Left }

#[derive(Copy, Clone)]
pub enum Tilt { Up, Down }

#[derive(Copy, Clone)]
pub enum MoveX { Pos, Neg, }

#[derive(Copy, Clone)]
pub enum MoveZ { Pos, Neg }

#[derive(Copy, Clone)]
pub enum Zoom { In, Out }

const D_POS: f32 = 0.05;
const D_PAN: Rad<f32> = Rad(1.0);
const D_TILT: Rad<f32> = Rad(1.0); 
const R_DIST: f32 = 0.05;

/* Default values for View members. */
const DEF_POS:  Vector3<f32> = Vector3 { x: 0.0, y: 0.0, z: 0.0};
const DEF_PAN:  Rad<f32> = Rad(0.0);
const DEF_TILT: Rad<f32> = Rad(0.0);
const DEF_DIST: f32 = 3.0;

/* Hold camera parameters. */
pub struct View {
    pos: Vector3<f32>,
    pan: Rad<f32>,
    tilt: Rad<f32>,
    dist: f32,
    view: Matrix4<f32>, // dont access this directly
}

impl View {
    pub fn new() -> View {
        View {
            pos: DEF_POS,
            pan: DEF_PAN,
            tilt: DEF_TILT,
            dist: DEF_DIST,
            view: View::construct_view(DEF_POS, DEF_PAN, DEF_TILT, DEF_DIST)
        }
    }

    /* Update view matrix. */
    pub fn update_view(&mut self) {
        self.view = View::construct_view(self.pos, self.pan, self.tilt, self.dist)
    }

    pub fn view_matrix(&mut self) -> &Matrix4<f32> {
        &self.view
    }

    pub fn pan(&mut self, dir: Pan, dt: f32) {
        match dir {
            Pan::Left => self.pan -= D_PAN * dt,
            Pan::Right => self.pan += D_PAN * dt
        }
        self.pan %= Rad(2.0 * PI)
    }

    pub fn tilt(&mut self, dir: Tilt, dt: f32) {
        println!("dt {}", dt);
        match dir {
            Tilt::Down => self.tilt -= D_TILT * dt,
            Tilt::Up => self.tilt += D_TILT * dt
        }
        self.tilt = clamp(
            self.tilt, 
            Rad(-FRAC_PI_2 + 0.0001), 
            Rad(FRAC_PI_2 - 0.0001)
        )
    }

    pub fn movex(&mut self, dir: MoveX, dt: f32) {
        let xdir = vec3(self.view.x[0], 0.0, self.view.z[0]).normalize();
        match dir {
            MoveX::Neg => self.pos -= xdir * D_POS * dt,
            MoveX::Pos => self.pos += xdir * D_POS * dt
        }
    }

    pub fn movez(&mut self, dir: MoveZ, dt: f32) {
        let zdir = vec3(self.view.x[2], 0.0, self.view.z[2]).normalize();
        match dir {
            MoveZ::Neg => self.pos -= zdir * D_POS * dt,
            MoveZ::Pos => self.pos += zdir * D_POS * dt
        }
    }

    pub fn zoom(&mut self, dir: Zoom, dt: f32) {
        let rt = R_DIST.powf(dt);
        self.dist *= match dir {
            Zoom::In => (1.0 - R_DIST),
            Zoom::Out => (1.0 + R_DIST),
        } 
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

    
}

fn clamp<T: PartialOrd>(value: T, lbound: T, ubound: T) -> T {
        if value < lbound {
            lbound
        } else if value > ubound {
            ubound
        } else {
            value
        }
    }