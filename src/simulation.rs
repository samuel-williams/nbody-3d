use cgmath::prelude::*;

use crate::render::Instance;

const G: f64 = 0.00000001;

const BODY_COLOR: [f32; 4] = [0.0, 0.2, 0.60, 1.0];

#[derive(Clone, Debug)]
struct Body {
    position: cgmath::Vector3<f64>,
    velocity: cgmath::Vector3<f64>,
    mass: f64,
}

impl PartialEq for Body {
    // for now, just check that they don't have the exact same position.
    // this would be a singularity anyway (infinite Fg).
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

enum SimulationBuffer {
    Buffer0,
    Buffer1,
}

pub enum BodyMass {
    Small,
    Medium,
    Large,
}

pub struct Simulation {
    buffer0: Vec<Body>,
    buffer1: Vec<Body>,
    current_buffer: SimulationBuffer,
}

fn barycenter_for_bodies(bodies: &Vec<Body>) -> cgmath::Vector3<f64> 
{
    let total_mass: f64 = bodies.iter().map(|b| b.mass).sum();    
    bodies.iter().map(|b| b.mass * b.position).sum::<cgmath::Vector3<f64>>() / total_mass
}

fn gravitational_force(a: &Body, b: &Body) -> f64 {
    let displacement = a.position - b.position;
    G * a.mass * b.mass / displacement.magnitude2()
}

fn orbital_velocity(a: &Body, b: &Body) -> cgmath::Vector3<f64> {
    let gravitational_parameter = G * (a.mass + b.mass);
    let displacement = b.position - a.position;
    let v = (gravitational_parameter / displacement.magnitude()).sqrt();

    v * (b.mass / (a.mass + b.mass)) * displacement.normalize().cross(cgmath::Vector3::unit_z())
}

impl Simulation {
    pub fn new() -> Self {
        let body0 = Body {
            position: (10.0, 0.0, 0.0).into(),
            velocity: (0.0, 0.0, 0.0).into(),
            mass: 10000000.0,
        };

        let body1 = Body {
            position: -2.0 * body0.position,
            velocity: (0.0, 0.0, 0.0).into(),
            mass: body0.mass / 2.0,
        };

        let mut bodies = vec![body0, body1];
        bodies[0].velocity = orbital_velocity(&bodies[0], &bodies[1]);
        bodies[1].velocity = orbital_velocity(&bodies[1], &bodies[0]);

        Simulation {
            buffer0: bodies.clone(),
            buffer1: bodies,
            current_buffer: SimulationBuffer::Buffer0,
        }
    }

    pub fn barycenter(&self) -> cgmath::Vector3<f64> {
        barycenter_for_bodies(&self.current_buffer())
        // cgmath::Vector3::zero()
    }

    pub fn instances(&self) -> Vec<crate::render::Instance> {
        let buffer = match self.current_buffer {
            SimulationBuffer::Buffer0 => &self.buffer0,
            SimulationBuffer::Buffer1 => &self.buffer1,
        };

        buffer.iter().map(|body| {
            Instance {
                position: cgmath::vec3(body.position.x as f32, body.position.y as f32, body.position.z as f32),
                rotation: cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0)),
                color: BODY_COLOR,
                scale: (body.mass.log10() / 7.0) as f32,
            }
        }).collect::<Vec<_>>()
    }
    
    pub fn add_body_at_position(&mut self, barycentric_position: cgmath::Vector3<f64>, mass: BodyMass) {
        let mut new_body = Body {
            position: barycentric_position,
            velocity: cgmath::Vector3::zero(),
            mass: match mass {
                BodyMass::Small => 10.0,
                BodyMass::Medium => 1000.0,
                BodyMass::Large => 100000.0,
            },
        };

        // get current bodies and sort (greatest-to-least) by gravitational force at this point
        let mut existing_bodies = self.current_buffer().clone();
        existing_bodies.sort_unstable_by(|a, b| {
            let force_a = gravitational_force(a, &new_body);
            let force_b = gravitational_force(b, &new_body);
            force_b.partial_cmp(&force_a).unwrap()
        });

        // see if we can find a reasonably stable orbit, 
        // otherwise orbit the barycenter and hope for the best (or chaos)
        let orbit_target = match existing_bodies.len() {
            0 => None,
            1 => Some(&existing_bodies[0]),
            _ => {
                let attractor_separation = (existing_bodies[0].position - existing_bodies[1].position).magnitude();
                let viable_orbit_radius = attractor_separation / 5.0;

                let distance0 = (new_body.position - existing_bodies[0].position).magnitude();
                let distance1 = (new_body.position - existing_bodies[1].position).magnitude();

                if distance0.le(&viable_orbit_radius) {
                    Some(&existing_bodies[0])
                } else if distance1.le(&viable_orbit_radius) {
                    Some(&existing_bodies[1])
                } else {
                    None
                }
            }
        };

        new_body.velocity = match orbit_target {
            Some(target) => {
                orbital_velocity(&new_body, target) + target.velocity
            }
            None => {
                // pretend barycenter is a point mass
                let temp_barycenter = Body {
                    position: self.barycenter(),
                    velocity: cgmath::Vector3::zero(),
                    mass: self.current_buffer().iter().map(|b| b.mass).sum(),
                };

                orbital_velocity(&new_body, &temp_barycenter)
            }
        };

        self.buffer0.push(new_body.clone());
        self.buffer1.push(new_body);
    }

    fn current_buffer(&self) -> &Vec<Body> {
        match self.current_buffer {
            SimulationBuffer::Buffer0 => &self.buffer0,
            SimulationBuffer::Buffer1 => &self.buffer1,
        }
    }

    pub fn tick(&mut self) {
        let (current_buffer, next_buffer) = match self.current_buffer {
            SimulationBuffer::Buffer0 => (&self.buffer0, &mut self.buffer1),
            SimulationBuffer::Buffer1 => (&self.buffer1, &mut self.buffer0),
        };

        // this is not as accurate as it could be, but it is fast.
        // in the future, could consider sorting intermediate values to sum the smaller values first
        for (current, next) in current_buffer.iter().zip(next_buffer.iter_mut()) {
            next.velocity = current_buffer.iter().fold(current.velocity, |velocity_acc, b| {
                if b != current {
                    let displacement = b.position - current.position;
                    let force = gravitational_force(current, b);
                    let d_velocity = force / current.mass;

                    velocity_acc + d_velocity * displacement.normalize()
                } else {
                    velocity_acc
                }
            });

            next.position = current.position + next.velocity;
        }


        self.current_buffer = match self.current_buffer {
            SimulationBuffer::Buffer0 => SimulationBuffer::Buffer1,
            SimulationBuffer::Buffer1 => SimulationBuffer::Buffer0,
        };
    }

    fn _debug_print_simulation_frame(&self) {
        println!("BEGIN SIMULATION FRAME");
        for body in self.current_buffer() {
            println!("  {:?}", body);
        }
        println!("END SIMULATION FRAME");
    }
}