extern crate cgmath;

#[macro_use]
extern crate glium;

#[macro_use]
extern crate itertools;
extern crate rand;

mod context;
mod input;
mod math;
mod scene;
mod shaders;
mod simulation;
mod teapot;
mod teapot_obj;

use cgmath::{Rad, vec3, Vector3};
use glium::{glutin, Surface};
use std::{thread, time};
use teapot::Teapot;


fn main() {
    /* Setup window. */
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(1280, 720)
        .with_title("Glium");
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    /* Load geometry for teapot model. */
    let tp_geom = teapot::load_geometry(&display);

    /* Compile shaders. */
    let program = glium::Program::from_source(
        &display, shaders::VERT, shaders::FRAG, None).unwrap();

    /* Initialize app context. */
    let mut ctxt = context::Context::new();    

    /* Construct teapots. */
    let mut teapots = vec![ 
        Teapot::new(
            Vector3 { x: -1.0, y: 0.0, z: 0.0 },
            vec3(0.0, 0.0, 0.003),
            1.0
        ),
        Teapot::new(
            Vector3 { x: 1.0, y: 0.0, z: 0.0 },
            vec3(0.0, 0.0, -0.003),
            1.0
        ),
    ];

    /* Construct simulation. */
    let mut sim = simulation::Simulation::new_unary();
    for i in (0..100) {
        sim.add_rand();
    }

    let light = [-3.0, 1.5, 3.0f32];
    let mut closed = false;
    let mut running = false;
    let frame_budget = time::Duration::from_millis(16);
    while !closed {
        let mut target = display.draw();
        let then = time::Instant::now();
        
        /* Clear draw area. */
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        /* Construct perspective matrix. */
        let (width, height) = target.get_dimensions();
        let fovy = Rad(3.141592 / 3.0);
        let aspect = width as f32 / height as f32;
        let perspective: [[f32; 4]; 4] = 
            cgmath::perspective(fovy, aspect, 0.1, 1024.0).into();
        
        /* Construct view matrix. */
        let view: [[f32; 4]; 4] = (*ctxt.view_matrix()).into();

        /* Setup draw params. */
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test:   glium::draw_parameters::DepthTest::IfLess,
                write:  true,
                ..      Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        /* Update simulation. */
        if running {
            sim.tick();
        }

        /* Draw teapots. */
        for teapot in sim.teapots() {
            let model: [[f32; 4]; 4] = teapot.model_matrix().into();
            let uniforms = uniform! {
                model: model,
                view: view,
                perspective: perspective,
                u_light: light
            };
            target.draw((&tp_geom.pos, &tp_geom.norm), &tp_geom.ind,
                        &program, &uniforms, &params).unwrap();
        }

        /* Finish. */
        target.finish().unwrap();

        /* Handle events. */
        let result = ctxt.handle_events(&mut events_loop);
        closed = result.0;
        running = result.1;

        let now = time::Instant::now();
        let elapsed = now.duration_since(then);

        if elapsed < frame_budget {
            thread::sleep(frame_budget - elapsed);
        }
        else {
            println!("blew frame budget, elapsed: {}", 
                elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
        }
    }
}

