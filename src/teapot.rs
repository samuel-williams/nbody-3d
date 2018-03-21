/* Define teapot type. */
// extern crate cgmath;
// extern crate glium;
// extern crate rand;

use rand;
use std::f32::consts::PI;
use teapot_obj::*;

use cgmath::{InnerSpace, Matrix4, Rad, vec3, Vector3};
use cgmath::conv::array3;
use glium;

pub const G: f64 = 0.0000001;  // scale gravitational force 
pub const MAX_TRAIL_LEN: usize = 5000;

pub struct Geometry {
    pub pos:    glium::VertexBuffer<GlVertex>,
    pub norm:   glium::VertexBuffer<GlNormal>,
    pub ind:    glium::IndexBuffer<u16>
}

pub fn load_geometry(display: &glium::Display) -> Box<Geometry> {
    Box::new(Geometry {
        pos: glium::VertexBuffer::new(display, &VERTICES).unwrap(),
        norm: glium::VertexBuffer::new(display, &NORMALS).unwrap(),
        ind: glium::IndexBuffer::new(display,
                glium::index::PrimitiveType::TrianglesList, 
                &INDICES
            ).unwrap()    
    })
}

#[derive(Clone)]
pub struct Teapot {
    // Draw parameters
    pub pos:    Vector3<f32>,   // position in world
    pub rot:    Rad<f32>,       // current rotation angle
    pub tilt:   Rad<f32>,       // axial tilt
    pub scale:  f32,            // object scale
    pub color:  Vector3<f32>,   // color to draw surface

    // Physics parameters
    pub d_rot:  Rad<f32>,
    pub vel:    Vector3<f64>,   // velocity vector
    pub m_exp:  f64,            // mass = 10 ^ m_exp
    pub id:     u32,            // unique identifier

    // Path vertices
    pub path:   Vec<GlVertex>,
}

impl Teapot {
    pub fn new(pos: Vector3<f32>, vel: Vector3<f64>, 
               m_exp: f64, id: u32) -> Teapot {
        let mut rng = rand::thread_rng();
        use rand::distributions::{IndependentSample, Range};
        let red = Range::new(0.0, 1.0).ind_sample(&mut rng);
        let green = Range::new(0.0, 1.0 - red).ind_sample(&mut rng);
        let blue = 1.0 - (red + green);

        Teapot {
            pos: pos,
            rot: Rad(0.0),
            tilt: Rad(Range::new(-0.25, 0.25).ind_sample(&mut rng)),
            scale: (m_exp / 500.0) as f32,
            color: vec3(red, green, blue),
            d_rot: Rad(Range::new(-0.01, 0.02).ind_sample(&mut rng)),
            vel: vel,
            m_exp: m_exp,
            id: id,
            path: Vec::new()
        }
    }

    pub fn update(&mut self, drop_crumb: bool) {
        self.rot = (self.rot + self.d_rot) % Rad(PI * 2.0);
        self.pos += self.vel.cast().unwrap();

        if drop_crumb {
            self.path.insert(0, GlVertex { position: array3(self.pos) });
            if self.path.len() > MAX_TRAIL_LEN {
                self.path.pop();
            }
        }
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        let translate = Matrix4::from_translation(self.pos);
        let rotate = Matrix4::from_angle_y(self.rot);
        let tilt = Matrix4::from_angle_x(self.tilt);
        let scale = Matrix4::from_scale(self.scale);
        translate * tilt * rotate * scale
    }

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    /* Calculate gravitational interaction over vector of objects. */
    // fn calc_net_force(&mut self, teapots: &vec::Vec<Teapot>) {
    //     for teapot in teapots {
    //         self.vel += self.calc_force(&teapot);
    //     }
    // }

    /* Calculate change in velocity due to gravitational
    interaction with single object. */
    pub fn calc_dvel(t0: &Teapot, t1: &Teapot) -> Vector3<f64> {
        /* Don't interact with self. */
        if t0.id == t1.id {
            return vec3(0.0, 0.0, 0.0);
        }

        let v0: Vector3<f64> = t0.pos.cast().unwrap();
        let v1: Vector3<f64> = t1.pos.cast().unwrap();

        let disp = v1 - v0;
        let r2 = disp.magnitude2();

        let dv = G * 10.0f64.powf(t1.m_exp) / r2;

        dv * disp.normalize()
    }

    // pub fn get_trail_geom(&self) -> (glium::VertexBuffer<[f32; 3]>, glium::IndexBuffer<u16>) {
    //     (self.path, 
    // }
}