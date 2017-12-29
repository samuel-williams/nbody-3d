/* Do physics simulation on teapots. */

use cgmath::{InnerSpace, vec3, Vector3};
use itertools::Itertools;
use rand;
use rand::distributions::{IndependentSample, Range};
use teapot::{G, Teapot};


pub struct Simulation {
    teapots:    Vec<Teapot>,
    rng:        rand::ThreadRng
}

impl Simulation {
    pub fn new_unary() -> Simulation {
        Simulation {
            teapots: vec![Teapot::new(
                vec3(0.0, 0.0, 0.0), 
                vec3(0.0, 0.0, 0.0), 
                5.0
            )],
            rng: rand::thread_rng()
        }
    }

    pub fn add_rand(&mut self) {
        let pos_range = Range::new(-7.0, 7.0f32);
        let px = pos_range.ind_sample(&mut self.rng);
        let py = Range::new(-0.5, 0.5f32).ind_sample(&mut self.rng);
        let pz = pos_range.ind_sample(&mut self.rng);
        let pos = vec3(px, py, pz);
        let mut teapot = Teapot::new(pos, vec3(0.0, 0.0, 0.0), 1.0);
        let vel = Simulation::orbit_vel(&teapot, self.greatest_mass());
        teapot.vel = 0.85 * vel;
        println!("pos {} {} {}", pos.x, pos.y, pos.z);
        println!("vel {} {} {}", vel.x, vel.y, vel.z);
        self.teapots.push(teapot);
    }

    /* Get teapots. */
    pub fn teapots(&self) -> &Vec<Teapot> {
        &self.teapots
    }

    /* Process one tick of the physics simulation. */
    pub fn tick(&mut self) {
        /* TODO: fix this wacky copy stuff. */
        let copy = self.teapots.to_vec();
        for teapot in self.teapots.iter_mut() {
            for other in copy.iter() {
                let dvel = Teapot::calc_dvel(&teapot, &other);
                teapot.vel += dvel;
            }
            teapot.update();
        }
    }

    /* Return position and magnitude of system barycenter. */
    fn greatest_mass(&self) -> &Teapot {
        let mut max = &self.teapots[0];
        for teapot in &self.teapots {
            if teapot.m_exp > max.m_exp {
                max = &teapot;
            }
        }
        max
    }

    /* Attempt to calculate stable orbit for obj1 about obj2. */
    fn orbit_vel(obj1: &Teapot, obj2: &Teapot) -> Vector3<f64> {
        let m1m2 = 10.0f64.powf(obj1.m_exp) + 10.0f64.powf(obj2.m_exp);
        let disp: Vector3<f64> = (obj1.pos - obj2.pos).cast();

        let v = (G * m1m2 / disp.magnitude()).sqrt();
        let dir = disp.cross(vec3(0.0, 1.0, 0.0)).normalize();
        v * dir
    }
}