/* Define teapot type. */
extern crate cgmath;
extern crate glium;
// extern crate rand;

use rand;
use std::f32::consts::PI;
use std::vec;
use teapot_obj;

use cgmath::{InnerSpace, Matrix4, Rad, vec3, Vector3};

pub const G: f64 = 0.0000001;  // scale gravitational force

pub struct Geometry {
    pub pos:    glium::VertexBuffer<teapot_obj::Vertex>,
    pub norm:   glium::VertexBuffer<teapot_obj::Normal>,
    pub ind:    glium::IndexBuffer<u16>
}

pub fn load_geometry(display: &glium::Display) -> Box<Geometry> {
    Box::new(Geometry {
        pos: glium::VertexBuffer::new(
            display, &teapot_obj::VERTICES).unwrap(),
        norm: glium::VertexBuffer::new(
            display, &teapot_obj::NORMALS).unwrap(),
        ind: glium::IndexBuffer::new(display, 
            glium::index::PrimitiveType::TrianglesList, 
            &teapot_obj::INDICES).unwrap()    
    })
}

#[derive(Copy, Clone)]
pub struct Teapot {
    // World parameters
    pub pos:    Vector3<f32>,   // position in world
    pub rot:    Rad<f32>,       // current rotation angle
    pub tilt:   Rad<f32>,       // axial tilt
    pub scale:  f32,            // object scale

    // Physics parameters
    pub d_rot:  Rad<f32>,
    pub vel:    Vector3<f64>,   // velocity vector
    pub m_exp:  f64,            // mass = 10 ^ m_exp
}

impl Teapot {
    pub fn new(pos: Vector3<f32>, vel: Vector3<f64>, m_exp: f64) -> Teapot {
        let mut rng = rand::thread_rng();
        use rand::distributions::{IndependentSample, Range};
        Teapot {
            pos: pos,
            rot: Rad(0.0),
            tilt: Rad(Range::new(-0.25, 0.25).ind_sample(&mut rng)),
            scale: (m_exp / 500.0) as f32,
            d_rot: Rad(Range::new(-0.01, 0.02).ind_sample(&mut rng)),
            vel: vel,
            m_exp: m_exp
        }
    }

    pub fn update(&mut self) {
        self.rot = (self.rot + self.d_rot) % Rad(PI * 2.0);
        self.pos += self.vel.cast();
    }

    pub fn model_matrix(&self) -> Matrix4<f32> {
        let translate = Matrix4::from_translation(self.pos);
        let rotate = Matrix4::from_angle_y(self.rot);
        let tilt = Matrix4::from_angle_x(self.tilt);
        let scale = Matrix4::from_scale(self.scale);
        translate * tilt * rotate * scale
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
        if t0.pos == t1.pos {
            return vec3(0.0, 0.0, 0.0);
        }

        let v0: Vector3<f64> = t0.pos.cast();
        let v1: Vector3<f64> = t1.pos.cast();

        // println!("pos: {} {} {}", self.pos.x, self.pos.y, self.pos.z);
        // println!("v0: {} {} {}", v0.x, v0.y, v0.z);

        // println!("pos: {} {} {}", teapot.pos.x, teapot.pos.y, teapot.pos.z);
        // println!("v0: {} {} {}", v1.x, v1.y, v1.z);

        let disp = v1 - v0;
        // println!("disp: {} {} {}", disp.x, disp.y, disp.z);
        let r2 = disp.magnitude2();
        // println!("r: {}", r);

        let dv = G * 10.0f64.powf(t1.m_exp) / r2;
        // println!("dv: {}", dv);
        // println!("m_exp0: {}  m_exp1: {}", t0.m_exp, t1.m_exp);

        // println!("m0: {}  m1: {}  r: {}  r2: {}", 
            // 10.0f64.powf(t0.m_exp), 10.0f64.powf(t1.m_exp),
            // disp.magnitude(), r2);
        // println!("dv: {}", dv);

        dv * disp.normalize()
    }
}