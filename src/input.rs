/* Handle input to simulator. */

use glium::glutin::{
    self, 
    ElementState,
    EventsLoop, 
    VirtualKeyCode
};

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

/* Encapsulate mouse press data. */
#[derive(Copy, Clone)]
pub struct MousePress {
    pub pos: (f64, f64),
}

pub struct Input {
    // movement state
    pub move_x: Option<MoveX>,
    pub move_z: Option<MoveZ>,
    pub pan: Option<Pan>,
    pub tilt: Option<Tilt>,
    pub zoom: Option<Zoom>,
    // pub menu: bool,

    // mouse state
    pub mouse_press: Option<MousePress>,
    m_pos: (f64, f64),
    m_in_window: bool,

    // program state
    pub running: bool,  /* Play/pause simulation. */
    pub trails: bool,   /* Toggle trails. */
    pub clear: bool,    /* Clear trails. */
    pub add: bool,      /* Add random. */
    pub close: bool,    /* Quit simulation. */     
}

impl Input {
    pub fn new() -> Input {
        Input {
            // keyboard
            move_x: None,
            move_z: None,
            pan: None,
            tilt: None,
            zoom: None,
            running: false,
            trails: true,
            clear: false,
            add: false,
            close: false,
            // menu: true,

            // mouse
            mouse_press: None,
            m_pos: (0.0, 0.0),
            m_in_window: false,
        }
    }

    pub fn handle_events(&mut self, events_loop: &mut EventsLoop) {
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => self.close = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } => 
                        self.handle_key(input),
                    glutin::WindowEvent::CursorEntered { .. } => 
                        self.m_in_window = true,
                    glutin::WindowEvent::CursorLeft { .. } => 
                        self.m_in_window = false,
                    glutin::WindowEvent::CursorMoved { position, .. } => 
                        self.m_pos = position,

                    /* Can't decode mouse events at this level for now. */
                    // glutin::WindowEvent::MouseInput { state, button, .. } =>
                    //     if state == glutin::ElementState::Pressed {
                    //         self.mouse_press = Some(MousePress { pos: self.m_pos })
                    //     },
                    
                    _ => (),
                },
                _ => (),
            }
        })
    }

    fn handle_key(&mut self, input: glutin::KeyboardInput) {
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

            Some(VirtualKeyCode::C) => {
                if input.state == ElementState::Released {
                    self.clear = true
                }
            },

            Some(VirtualKeyCode::R) => {
                if input.state == ElementState::Released {
                    self.add = true
                }
            },

            // Some(VirtualKeyCode::M) => {
            //     use menu;
            //     menu::open();
            //     self.menu = true;
            // },

            _ => (),
        }
    }

    // fn handle_mouse(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
    //     use glutin::{MouseButton, ElementState};

    //     // only LMB press for now.
    //     if (state, button) == (ElementState::Pressed, MouseButton::Left) {

    //     }
    // }
}