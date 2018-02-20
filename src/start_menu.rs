/*  
    Menu that is displayed when starting the application. 
    Gives user choice of several simulation templates:
        binary - system with two large masses orbiting one another
        unary - system with single, stationary mass
        TODO: cloud - system with many small masses (orbiting barycenter?)
*/

use conrod::{self, widget, Colorable, Positionable, Widget};
use conrod::backend::glium::glium::{self, Surface};

use simulation_window::Template;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

widget_ids!(struct Ids { 
    master,
    header,
    header_text,
    body, 
    run_toggle,
    trails_toggle,
    clear_button,
});

struct MenuState {
    running: bool,
    template_selection: Option<Template>,
}

pub struct StartMenu {
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
    ui: conrod::Ui,
    ids: Ids,
    renderer: conrod::backend::glium::Renderer,
    image_map: conrod::image::Map<glium::texture::Texture2d>,
    state: MenuState,
}

impl StartMenu {
    pub fn new() -> Self {
        /* Setup window. */
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("nbody-3D menu")
            .with_dimensions(WIDTH, HEIGHT)
            .with_max_dimensions(WIDTH, HEIGHT)
            .with_min_dimensions(WIDTH, HEIGHT);
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

        StartMenu {
            events_loop: events_loop,
            display: display,
            ui: ui,
            ids: ids,
            renderer: renderer,
            image_map: image_map,
            state: MenuState {
                running: true,
                template_selection: None,
            },
            // rx_port: rx,
        }
    }

    pub fn execute(&mut self) -> Option<Template> {
        let mut events = Vec::new();

        'render: while self.state.running {
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
                            } => break 'render,
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

        self.state.template_selection
    }

    fn set_widgets(&mut self) {
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

        let ui = &mut self.ui.set_widgets();
        let ids = &self.ids;
        let state = &mut self.state;

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
        let binary_button = widget::Button::new()
            .color(color::DARK_GREEN)
            .w_h(100.0, 100.0)
            .label("Binary")
            .mid_left_with_margin_on(ids.body, 20.0);

        let unary_button = widget::Button::new()
            .color(color::DARK_GREEN)
            .w_h(100.0, 100.0)
            .label("Unary")
            .middle_of(ids.body);
            

        let cloud_button = widget::Button::new()
            .color(color::DARK_GREEN)
            .w_h(100.0, 100.0)
            .label("Cloud")
            .mid_right_with_margin_on(ids.body, 20.0);

        for _ in binary_button.set(ids.run_toggle, ui) {
            state.template_selection = Some(Template::Binary);
            state.running = false;
        }

        for _ in unary_button.set(ids.trails_toggle, ui) {
            state.template_selection = Some(Template::Unary);
            state.running = false;
        }

        for _ in cloud_button.set(ids.clear_button, ui) {
            state.template_selection = Some(Template::Cloud);
            state.running = false;
        }

    }
}