extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate gl;

use piston::window::WindowSettings;
use piston::window::OpenGLWindow;
use piston::event_loop::*;
use piston::input::*;
use glfw_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend
    rotation: f64   // Rotation for the square
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (
            (args.width / 2) as f64,
            (args.height / 2) as f64
        );

        self.gl.draw(args.viewport(), |c, gl| {
            /* Clear screen. */
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            /* Draw a box rotating around the middle of the screen. */
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        /* Rotate 2 radians per second. */
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    /* Greate a Glutin window. */
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    /* Load OpenGL function pointers. */
    gl::load_with(|s| window.get_proc_address(s));
    
    let mut gl = GlGraphics::new(opengl);

    /* Create a new game and run it. */
    // let mut app = App {
    //     gl: GlGraphics::new(opengl),
    //     rotation: 0.0
    // };

    // let mut events = Events::new(EventSettings::new());
    // while let Some(e) = events.next(&mut window) {
    //     if let Some(r) = e.render_args() {
    //         app.render(&r);
    //     }

    //     if let Some(u) = e.update_args() {
    //         app.update(&u);
    //     }
    // }
}

// extern crate cgmath;

// // #[cfg(all(feature="winit", feature="glium"))] 
// #[macro_use] 
// extern crate conrod;

// extern crate rusttype;

// #[macro_use]
// extern crate glium;

// // #[macro_use]
// // extern crate itertools;
// extern crate rand;

// mod context;
// mod gl_vertex;
// mod input;
// mod math;
// mod scene;
// mod shaders;
// mod simulation;
// mod teapot;
// mod teapot_obj;

// use conrod::{widget, Colorable, Positionable, Widget};
// use cgmath::Rad;
// use cgmath::conv::*;
// use glium::{glutin, Surface};
// use glutin::WindowBuilder;
// use gl_vertex::{GlNormal, GlVertex};
// use std::{thread, time};
// // use teapot::Teapot;


// fn main() {
//     /* Setup window. */
//     let mut events_loop = glutin::EventsLoop::new();
//     let window = WindowBuilder::new()
//         .with_dimensions(1280, 720)
//         .with_title("Glium");
//     let context = glutin::ContextBuilder::new()
//         .with_depth_buffer(24)
//         .with_multisampling(0);
//     let display = glium::Display::new(window, context, &events_loop).unwrap();

//     /* Load geometry for teapot model. */
//     let tp_geom = teapot::load_geometry(&display);

//     /* Compile shaders. */
//     let tri_prog = glium::Program::from_source(
//         &display, shaders::TRI_VERT, shaders::TRI_FRAG, None).unwrap();
//     let line_prog = glium::Program::from_source(
//         &display, shaders::LINE_VERT, shaders::LINE_FRAG, None).unwrap();

//     /* Initialize app context. */
//     let mut ctxt = context::Context::new();

//     /* Construct simulation. */
//     let mut sim = simulation::Simulation::new_unary();
//     for _ in 0..200 {
//         sim.add_rand();
//     }

//     let light = [-3.0, 1.5, 3.0f32];
//     let mut closed = false;
//     let mut running = false;
//     let mut trails = true;
//     let frame_budget = time::Duration::from_millis(16);
//     while !closed {
//         let mut target = display.draw();
//         let then = time::Instant::now();
        
//         /* Clear draw area. */
//         target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

//         /* Construct perspective matrix. */
//         let (width, height) = target.get_dimensions();
//         let fovy = Rad(3.141592 / 3.0);
//         let aspect = width as f32 / height as f32;
//         let perspective: [[f32; 4]; 4] = 
//             cgmath::perspective(fovy, aspect, 0.1, 1024.0).into();
        
//         /* Construct view matrix. */
//         let view: [[f32; 4]; 4] = (*ctxt.view_matrix()).into();

//         /* Setup draw params. */
//         let tri_params = glium::DrawParameters {
//             depth: glium::Depth {
//                 test:   glium::draw_parameters::DepthTest::IfLess,
//                 write:  true,
//                 ..      Default::default()
//             },
//             //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
//             .. Default::default()
//         };

//         let line_params = glium::DrawParameters {
//             depth: glium::Depth {
//                 test:   glium::draw_parameters::DepthTest::IfLess,
//                 write:  true,
//                 ..      Default::default()
//             },
//             .. Default::default()
//         };

//         /* Update simulation. */
//         if running {
//             sim.tick();
//         }

//         /* Draw teapots. */
//         for teapot in sim.teapots() {
//             let model: [[f32; 4]; 4] = teapot.model_matrix().into();
//             let uniforms = uniform! {
//                 model: model,
//                 view: view,
//                 perspective: perspective,
//                 u_light: light,
//                 surface_color: array3(teapot.color)
//             };
//             target.draw((&tp_geom.pos, &tp_geom.norm), &tp_geom.ind,
//                         &tri_prog, &uniforms, &tri_params).unwrap();

//             /* Draw path if enabled. */
//             if trails {
//                 let line_uniforms = uniform! {
//                     view: view,
//                     perspective: perspective,
//                     surface_color: array3(teapot.color)
//                 };

//                 let pos = glium::VertexBuffer::new(&display, &teapot.path).unwrap();
//                 let ind = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);

//                 target.draw(&pos, &ind, &line_prog, &line_uniforms, &line_params).unwrap();
//             }
//         }

//         /* Finish. */
//         target.finish().unwrap();

//         /* Handle events. */
//         let result = ctxt.handle_events(&mut events_loop);
//         closed = result.0;
//         running = result.1;
//         trails = result.2;

//         let now = time::Instant::now();
//         let elapsed = now.duration_since(then);

//         if elapsed < frame_budget {
//             thread::sleep(frame_budget - elapsed);
//         }
//         else {
//             println!("blew frame budget, elapsed: {}", 
//                 elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9);
//         }
//     }
// }

