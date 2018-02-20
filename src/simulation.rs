/* Do physics simulation on teapots. */

use cgmath::{InnerSpace, vec3, Vector3};
// use itertools::Itertools;
use rand;
use rand::distributions::{IndependentSample, Range};
use rayon::prelude::*;
use teapot::{G, Teapot};

const TICKS_PER_PATH_VERT: u64 = 1;

pub enum Orbit {
    GreatestMass,
    GreatestForce,
    Barycenter,
}

pub struct Simulation {
    teapots:    Vec<Teapot>,
    rng:        rand::ThreadRng,
    tick:       u64,                // current tick
    next_id:    u32,                // unique id for next object
}

impl Simulation {
    pub fn new_unary() -> Simulation {
        Simulation {
            teapots: vec![Teapot::new(
                vec3(0.0, 0.0, 0.0), 
                vec3(0.0, 0.0, 0.0), 
                5.0,
                0,
            )],
            rng: rand::thread_rng(),
            tick: 0,
            next_id: 1
        }
    }

    pub fn new_binary(m_exp: f64, r: f32) -> Simulation {
        let mut b0 = Teapot::new(
            vec3(r, 0.0, 0.0), 
            vec3(0.0, 0.0, 0.0), 
            m_exp, 
            0
        );
        let mut b1 = Teapot::new(
            vec3(-r, 0.0, 0.0), 
            vec3(0.0, 0.0, 0.0), 
            m_exp, 
            1
        );

        b0.vel = Simulation::orbit_vel(&b0, &b1) / 2.0;
        b1.vel = Simulation::orbit_vel(&b1, &b0) / 2.0;

        Simulation {
            teapots: vec![b0, b1],
            rng: rand::thread_rng(),
            tick: 0,
            next_id: 2
        }
    }

    pub fn add_rand(&mut self, orbit: Orbit, ecc: f64) {
        let pos_range = Range::new(-7.0, 7.0f32);
        let px = pos_range.ind_sample(&mut self.rng);
        let py = Range::new(-0.5, 0.5f32).ind_sample(&mut self.rng);
        let pz = pos_range.ind_sample(&mut self.rng);
        let pos = vec3(px, py, pz);
        let id = self.next_id;
        self.next_id += 1;
        let mut teapot = Teapot::new(pos, vec3(0.0, 0.0, 0.0), 1.0, id);
        teapot.vel = match orbit {
            Orbit::GreatestForce => {
                let target = self.greatest_force(&teapot);
                ecc * Simulation::orbit_vel(&teapot, target) //+ target.vel
            },
            Orbit::GreatestMass => {
                let target = self.greatest_mass();
                ecc * Simulation::orbit_vel(&teapot, target)
            },
            Orbit::Barycenter => unimplemented!()
        };

        // println!("pos {} {} {}", pos.x, pos.y, pos.z);
        // println!("vel {} {} {}", vel.x, vel.y, vel.z);
        // println!("id  {}", id);
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
            teapot.update(self.tick % TICKS_PER_PATH_VERT == 0);
        }

        self.tick += 1;
    }

    /* Find object with greatest mass. */
    fn greatest_mass(&self) -> &Teapot {
        let mut max = &self.teapots[0];
        for teapot in &self.teapots {
            if teapot.m_exp > max.m_exp {
                max = &teapot;
            }
        }

        println!("greatest mass id: {}", max.id);
        max
    }

    /* Find object exerting greatest force on given object. */
    fn greatest_force(&self, obj: &Teapot) -> &Teapot {
        let mut max = &self.teapots[0];
        let mut max_accel = Teapot::calc_dvel(max, obj);
        for teapot in &self.teapots {
            let accel = Teapot::calc_dvel(teapot, obj);
            if accel.magnitude() > max_accel.magnitude() {
                max_accel = accel;
                max = teapot;
            }
        }

        // if max.id != 0 {
        //     println!("target:  id {}  force {}", max.id, Teapot::calc_dvel(max, obj).magnitude());
        //     println!("m0 force: {}", Teapot::calc_dvel(&self.teapots[0], obj).magnitude());
        // }

        max
    }

    /* Attempt to calculate stable orbit for obj1 about obj2. */
    fn orbit_vel(obj1: &Teapot, obj2: &Teapot) -> Vector3<f64> {
        let m1m2 = 10.0f64.powf(obj1.m_exp) + 10.0f64.powf(obj2.m_exp);
        let disp: Vector3<f64> = (obj1.pos - obj2.pos).cast().unwrap();

        let v = (G * m1m2 / disp.magnitude()).sqrt();
        let dir = disp.cross(vec3(0.0, 1.0, 0.0)).normalize();
        v * dir
    }

    pub fn clear_paths(&mut self) {
        for mut teapot in &mut self.teapots {
            teapot.clear_path();
        }
    }
}