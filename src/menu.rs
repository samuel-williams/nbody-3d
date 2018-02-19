use conrod::{self, widget, Colorable, Positionable, Widget};
use conrod::backend::glium::glium::{self, Surface};

use std::sync::mpsc;
use std::thread;
use std::vec::Vec;

pub enum MenuMsg {
    ToggleRun,
    ToggleTrails,
    Clear,
    Close,
}

pub struct MenuHandle {
    // tx: mpsc::Sender<MenuMsg>,
    rx: mpsc::Receiver<MenuMsg>,
}

impl MenuHandle {
    pub fn build() -> MenuHandle {
        /* IPC */
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            Menu::init(tx).run()
        });

        MenuHandle {
            rx: rx,
        }
    }

    pub fn get_messages(&self) -> Vec<MenuMsg> {
        let mut messages = Vec::new();
        while let Ok(msg) = self.rx.try_recv() {
            messages.push(msg);
        }

        messages
    }
}

const WIDTH: u32 = 800;
const HEIGHT: u32 = 400;


struct MenuState {
    running: bool,
    trails: bool,
}

struct Menu {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    ui: conrod::Ui,
    ids: Ids,
    renderer: conrod::backend::glium::Renderer,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    state: MenuState,
    tx_port: mpsc::Sender<MenuMsg>,
    // rx_port: mpsc::Receiver<MenuMsg>,
}

impl Menu {
    fn init(tx: mpsc::Sender<MenuMsg>, /* rx: mpsc::Receiver<MenuMsg> */) -> Self {
        /* Setup window. */
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("NBody-3D Menu")
            .with_dimensions(WIDTH, HEIGHT);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        /* Construct our `Ui`. */
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

        /* Generate the widget identifiers. */        
        let ids = Ids::new(ui.widget_id_generator());

        /* Add a `Font` to the `Ui`'s `font::Map` from file. */
        const FONT_PATH: &'static str = concat!(
                env!("CARGO_MANIFEST_DIR"), 
                "/assets/fonts/NotoSans/NotoSans-Regular.ttf"
            );
        ui.fonts.insert_from_file(FONT_PATH).unwrap();

        /* A type used for converting `conrod::render::Primitives` into 
        `Command`s that can be used for drawing to the glium `Surface`. */
        let mut renderer = conrod::backend::glium::Renderer::new(&display)
            .unwrap();

        /* The image map describing each of our widget->image mappings 
        (in our case, none). */
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

        Menu {
            events_loop: events_loop,
            display: display,
            ui: ui,
            ids: ids,
            renderer: renderer,
            image_map: image_map,
            state: MenuState {
                running: false,
                trails: true,
            },
            tx_port: tx,
            // rx_port: rx,
        }
    }

    fn run(&mut self) {
        let mut events = Vec::new();

        'render: loop {
            events.clear();

            // Get all the new events since the last frame.
            self.events_loop.poll_events(|event| { events.push(event); });

            // If there are no new events, wait for one.
            if events.is_empty() {
                self.events_loop.run_forever(|event| {
                    events.push(event);
                    glium::glutin::ControlFlow::Break
                });
            }

            // Process the events.
            for event in events.drain(..) {

                // Break from the loop upon `Escape` or closed window.
                match event.clone() {
                    glium::glutin::Event::WindowEvent { event, .. } => {
                        match event {
                            glium::glutin::WindowEvent::Closed |
                            glium::glutin::WindowEvent::KeyboardInput {
                                input: glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => {
                                self.tx_port.send(MenuMsg::Close);
                                break 'render
                            },
                            _ => (),
                        }
                    }
                    _ => (),
                };

                // Use the `winit` backend feature to convert the winit event to a conrod input.
                let input = match conrod::backend::winit::convert_event(event, &self.display) {
                    None => continue,
                    Some(input) => input,
                };

                // Handle the input with the `Ui`.
                self.ui.handle_event(input);

                // Set the widgets.
                
                self.set_widgets();

                // "Hello World!" in the middle of the screen.
                // widget::Text::new("Hello World!")
                //     .middle_of(ui.window)
                //     .color(conrod::color::WHITE)
                //     .font_size(32)
                //     .set(self.ids.text, ui);
            }

            // Draw the `Ui` if it has changed.
            if let Some(primitives) = self.ui.draw_if_changed() {
                self.renderer.fill(&self.display, primitives, &self.image_map);
                let mut target = self.display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                self.renderer.draw(&self.display, &mut target, &self.image_map)
                    .unwrap();
                target.finish().unwrap();
            }
        }
    }

    fn set_widgets(&mut self) {//ref mut ui: conrod::UiCell, ids: &mut Ids, state: &mut MenuState) {
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

        let ui = &mut self.ui.set_widgets();
        let ids = &self.ids;
        let state = &mut self.state;
        let tx_port = &self.tx_port;

        /* Construct canvas tree. */
        widget::Canvas::new().flow_down(&[
            (ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
            (ids.body, widget::Canvas::new().color(color::DARK_BLUE))
        ]).set(ids.master, ui);

        /* Header text. */
        widget::Text::new("nbody-3D!")
            .color(color::BLUE.complement())
            .font_size(48)
            .middle_of(ids.header)
            .set(ids.header_text, ui);

        /* Buttons. */
        let run_toggle = widget::Toggle::new(state.running)
            .color(color::DARK_GREEN)
            .w_h(100.0, 100.0)
            .label("Run")
            .mid_left_with_margin_on(ids.body, 20.0);

        let trails_toggle = widget::Toggle::new(state.trails)
            .color(color::DARK_GREEN)
            .w_h(100.0, 100.0)
            .label("Trails")
            .mid_right_with_margin_on(ids.body, 20.0);

        let clear_button = widget::Button::new()
            .color(color::DARK_GREEN)
            .w_h(100.0, 100.0)
            .label("Clear")
            .middle_of(ids.body);

        for _ in run_toggle.set(ids.run_toggle, ui) {
            state.running = !state.running;
            self.tx_port.send(MenuMsg::ToggleRun).unwrap();
        }

        for _ in trails_toggle.set(ids.trails_toggle, ui) {
            state.trails = !state.trails;
            self.tx_port.send(MenuMsg::ToggleTrails).unwrap();
        }

        for _ in clear_button.set(ids.clear_button, ui) {
            self.tx_port.send(MenuMsg::Clear).unwrap();
        }

    }
}

widget_ids!(struct Ids { 
    master,
    header,
    header_text,
    body, 
    run_toggle,
    trails_toggle,
    clear_button,
});