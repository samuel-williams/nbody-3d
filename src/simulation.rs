/* Do physics simulation on teapots. */

use cgmath::{InnerSpace, vec3, Vector3};
// use itertools::Itertools;
use rand;
use rand::distributions::{IndependentSample, Range};
use teapot::{G, Teapot};

const TICKS_PER_PATH_VERT: u64 = 5;

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

    pub fn new_binary() -> Simulation {
        Simulation {
            teapots: vec![
                Teapot::new(
                    vec3(1.0, 0.0, 0.0), 
                    vec3(0.0, 0.0, 0.01), 
                    3.5,
                    0,
                ),
                Teapot::new(
                    vec3(-1.0, 0.0, 0.0), 
                    vec3(0.0, 0.0, -0.01), 
                    3.5,
                    1,
                )
            ],
            rng: rand::thread_rng(),
            tick: 0,
            next_id: 2
        }
    }

    pub fn add_rand(&mut self) {
        let pos_range = Range::new(-7.0, 7.0f32);
        let px = pos_range.ind_sample(&mut self.rng);
        let py = Range::new(-0.5, 0.5f32).ind_sample(&mut self.rng);
        let pz = pos_range.ind_sample(&mut self.rng);
        let pos = vec3(px, py, pz);
        let id = self.next_id;
        self.next_id += 1;
        let mut teapot = Teapot::new(pos, vec3(0.0, 0.0, 0.0), 1.0, id);
        let vel = Simulation::orbit_vel(&teapot, self.greatest_mass());
        teapot.vel = 1.2 * vel;
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