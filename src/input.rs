/* Handle input to simulator. */

use glium::glutin;
use glutin::EventsLoop;

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

pub struct Input {
    // movement state
    pub move_x: Option<MoveX>,
    pub move_z: Option<MoveZ>,
    pub pan:    Option<Pan>,
    pub tilt:   Option<Tilt>,
    pub zoom:   Option<Zoom>,

    // program state
    pub running:bool,
    pub trails: bool,
    pub close:  bool     
}

impl Input {
    pub fn new() -> Input {
        Input {
            move_x: None,
            move_z: None,
            pan:    None,
            tilt:   None,
            zoom:   None,
            // strafe: None,
            running:false,
            trails: true,
            close:  false
        }
    }

    pub fn handle_events(&mut self, events_loop: &mut EventsLoop) {
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => self.close = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } => 
                        self.handle_key(input),
                    _ => (),
                },
                _ => (),
            }
        })
    }

    fn handle_key(&mut self, input: glutin::KeyboardInput) {
        use glutin::{ElementState, VirtualKeyCode};

        /* Keymap. */
        match input.virtual_keycode {
            Some(VirtualKeyCode::Up) => self.tilt = match input.state {
                ElementState::Pressed =>    Some(Tilt::Up),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::Down) => self.tilt = match input.state {
                ElementState::Pressed =>    Some(Tilt::Down),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::Left) => self.pan = match input.state {
                ElementState::Pressed =>    Some(Pan::Left),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::Right) => self.pan = match input.state {
                ElementState::Pressed =>    Some(Pan::Right),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::W) => self.move_z = match input.state {
                ElementState::Pressed =>    Some(MoveZ::Neg),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::S) => self.move_z = match input.state {
                ElementState::Pressed =>    Some(MoveZ::Pos),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::A) => self.move_x = match input.state {
                ElementState::Pressed =>    Some(MoveX::Neg),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::D) => self.move_x = match input.state {
                ElementState::Pressed =>    Some(MoveX::Pos),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::Z) => self.zoom = match input.state {
                ElementState::Pressed =>    Some(Zoom::In),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::X) => self.zoom = match input.state {
                ElementState::Pressed =>    Some(Zoom::Out),
                ElementState::Released =>   None,
            },

            Some(VirtualKeyCode::Space) => { 
                if input.state == ElementState::Released {
                    self.running = !self.running
                }
            },

            Some(VirtualKeyCode::P) => {
                if input.state == ElementState::Released {
                    self.trails = !self.trails
                }
            },

            _ => (),
        }
    }
}