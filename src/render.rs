/* Encapsulate rendering details. */

use cgmath::{self, Matrix4, Rad};
use cgmath::conv::*;
use glium::{self, Display, Program, Surface}; 
use shaders;
use teapot;
use std::vec;
use time::{Duration, PreciseTime};

pub struct RenderState<'a> {
    pub draw_trails: bool,
    pub view_matrix: &'a Matrix4<f32>,
    // model_matrices: vec::Vec<&'a Matrix4<f32>>,
    pub light_pos: [f32; 3],
}

pub struct Renderer<'a> {
    display: Display,
    geometry: Box<teapot::Geometry>,
    tri_prog: Program,
    line_prog: Program,
    tri_params: glium::DrawParameters<'a>,
    line_params: glium::DrawParameters<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(display: Display) -> Self {
        /* Load geometry for teapot model. */
        let geometry = teapot::load_geometry(&display);

        /* Compile shaders. */
        let tri_prog = Program::from_source(
                &display, 
                shaders::TRI_VERT, 
                shaders::TRI_FRAG, 
                None
            ).unwrap();

        let line_prog = Program::from_source(
                &display, 
                shaders::LINE_VERT, 
                shaders::LINE_FRAG, 
                None
            ).unwrap();

        /* Setup draw params. */
        let tri_params = glium::DrawParameters {
            depth: glium::Depth {
                test:   glium::draw_parameters::DepthTest::IfLess,
                write:  true,
                ..      Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let line_params = glium::DrawParameters {
            depth: glium::Depth {
                test:   glium::draw_parameters::DepthTest::IfLess,
                write:  true,
                ..      Default::default()
            },
            .. Default::default()
        };

        Renderer {
            display: display,
            geometry: geometry,
            tri_prog: tri_prog,
            line_prog: line_prog,
            tri_params: tri_params,
            line_params: line_params,
        }
    }

    pub fn draw(&self, state: RenderState, teapots: &Vec<teapot::Teapot>) -> Duration {
        let mut target = self.display.draw();
        let then = PreciseTime::now();
        
        /* Clear draw area. */
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        /* Construct perspective matrix. */
        let (width, height) = target.get_dimensions();
        let fovy = Rad(3.141592 / 3.0);
        let aspect = width as f32 / height as f32;
        let perspective: [[f32; 4]; 4] = 
            cgmath::perspective(fovy, aspect, 0.1, 1024.0).into();
        
        /* Construct view matrix. */
        let view: [[f32; 4]; 4] = (*state.view_matrix).into();

        /* Draw teapots. */
        for teapot in teapots {
            let model: [[f32; 4]; 4] = teapot.model_matrix().into();
            let tri_uniforms = uniform! {
                model: model,
                view: view,
                perspective: perspective,
                u_light: state.light_pos,
                surface_color: array3(teapot.color)
            };

            target.draw(
                (&self.geometry.pos, &self.geometry.norm), 
                &self.geometry.ind,
                &self.tri_prog, 
                &tri_uniforms, 
                &self.tri_params
            ).unwrap();

            /* Draw path if enabled. */
            if state.draw_trails {
                let line_uniforms = uniform! {
                    view: view,
                    perspective: perspective,
                    surface_color: array3(teapot.color)
                };

                let pos = glium::VertexBuffer::new(&self.display, &teapot.path).unwrap();
                let ind = glium::index::NoIndices(glium::index::PrimitiveType::LineStrip);

                target.draw(
                    &pos, 
                    &ind, 
                    &self.line_prog, 
                    &line_uniforms, 
                    &self.line_params
                ).unwrap();
            }
        }

        /* Finish. */
        target.finish().unwrap();

        then.to(PreciseTime::now())
    }
}