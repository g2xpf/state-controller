#[macro_use]
extern crate state_controller;
extern crate image;

use glium::{glutin, Frame};
use state_controller::{
    primitive_shape::{Circle, Rectangle, Texture},
    utils::{EaseInOutSin, EaseOutBounce, Timer},
    Event, EventHandler, IntermediateState, Key, Parent, Receiver, Renderable, ShapeContainer,
    Shifter, State, Transition, TransitionFlow, Transitionable, Updatable, World,
};

struct Global {
    resolution: (i32, i32),
    _timer: Timer,
}

impl Renderable for Global {}
impl Updatable for Global {}
impl EventHandler for Global {}
impl State for Global {}

struct InitState {
    counter: u64,
    rectangle_container: ShapeContainer<Rectangle>,
    texture_container: ShapeContainer<Texture>,
}

impl Renderable for InitState {
    fn render(&self, _shifter: &Shifter, frame: &mut Frame) {
        self.rectangle_container
            .render(frame, &Default::default(), (0., 0.));
        self.texture_container
            .render(frame, &Default::default(), (0., 0.));
    }
}

impl Updatable for InitState {
    fn update(&mut self, shifter: &mut Shifter) {
        if self.counter >= 1000 {
            self.counter = 0;
            self.shift::<SecondState>(shifter);
        }
        self.counter += 1;
    }
}

impl EventHandler for InitState {
    fn handle(&mut self, event: &Event) {
        if event.window.close_requested || event.key(Key::Escape).is_pressed() {
            std::process::exit(0)
        }
        for rectangle in self.rectangle_container.iter_mut() {
            let dr = 0.03;
            if event.key(Key::Right).is_pressed() {
                rectangle.pos[0] += dr;
            }
            if event.key(Key::Left).is_pressed() {
                rectangle.pos[0] -= dr;
            }
            if event.key(Key::Up).is_pressed() {
                rectangle.pos[1] += dr;
            }
            if event.key(Key::Down).is_pressed() {
                rectangle.pos[1] -= dr;
            }
            if event.key(Key::L).is_pressed() {
                rectangle.angle -= dr;
            }
            if event.key(Key::H).is_pressed() {
                rectangle.angle += dr;
            }
            if event.key(Key::K).is_pressed() {
                self.counter = 10000;
            }
        }
    }
}

impl State for InitState {}

struct SecondState {
    counter: u64,
    circle_container: ShapeContainer<Circle>,
}

impl Renderable for SecondState {
    fn render(&self, shifter: &Shifter, frame: &mut Frame) {
        let parent = self.parent_ref::<Global>(shifter);
        self.circle_container
            .render(frame, &Default::default(), (0., 0.), parent.resolution);
    }
}

impl Updatable for SecondState {
    fn update(&mut self, shifter: &mut Shifter) {
        if self.counter >= 1000 {
            self.counter = 0;
            self.shift::<InitState>(shifter);
        }
        self.counter += 1;
    }
}

impl EventHandler for SecondState {
    fn handle(&mut self, event: &Event) {
        if event.window.close_requested || event.key(Key::Escape).is_pressed() {
            std::process::exit(0)
        }
        for circle in self.circle_container.iter_mut() {
            let dr = 0.03;
            if event.key(Key::Right).is_pressed() {
                circle.pos[0] += dr;
            }
            if event.key(Key::Left).is_pressed() {
                circle.pos[0] -= dr;
            }
            if event.key(Key::Up).is_pressed() {
                circle.pos[1] += dr;
            }
            if event.key(Key::Down).is_pressed() {
                circle.pos[1] -= dr;
            }
            if event.key(Key::J).is_pressed() {
                self.counter = 10000;
            }
        }
    }
}

impl State for SecondState {}

impl Receiver<SecondState> for InitState {
    type Message = ();

    fn receive(&mut self, _message: Self::Message) {}
}

impl Receiver<InitState> for SecondState {
    type Message = ();

    fn receive(&mut self, _message: Self::Message) {}
}

struct InitToSecond {
    timer: Timer,
    transition: Transition<InitState, SecondState>,
}

impl IntermediateState for InitToSecond {
    fn transition_location(&mut self) -> &mut dyn Transitionable {
        &mut self.transition
    }

    fn initialize(&mut self) {
        self.timer.start();
    }

    fn finalize(&mut self) {
        self.timer.stop();
    }

    fn update(&mut self) -> TransitionFlow {
        match self.timer.is_over() {
            Some(true) => TransitionFlow::Break,
            _ => TransitionFlow::Continue,
        }
    }

    fn render(&self, shifter: &Shifter, frame: &mut Frame) {
        let (from, to) = self.transition.borrow();
        let parent = from.parent_ref::<Global>(shifter);
        match self.timer.get_ratio_easing::<EaseInOutSin>() {
            Some(ratio) => {
                let ratio = ratio as f32;
                from.rectangle_container
                    .render(frame, &Default::default(), (ratio * 2., 0.));
                from.texture_container
                    .render(frame, &Default::default(), (ratio * 2., 0.));
                to.circle_container.render(
                    frame,
                    &Default::default(),
                    (ratio * 2. - 2.0, 0.),
                    parent.resolution,
                );
            }
            _ => {}
        }
    }
}

struct SecondToInit {
    timer: Timer,
    transition: Transition<SecondState, InitState>,
}

impl IntermediateState for SecondToInit {
    fn transition_location(&mut self) -> &mut dyn Transitionable {
        &mut self.transition
    }

    fn initialize(&mut self) {
        self.timer.start();
    }

    fn finalize(&mut self) {
        self.timer.stop();
    }

    fn update(&mut self) -> TransitionFlow {
        match self.timer.is_over() {
            Some(true) => TransitionFlow::Break,
            _ => TransitionFlow::Continue,
        }
    }

    fn render(&self, shifter: &Shifter, frame: &mut Frame) {
        let (from, to) = self.transition.borrow();
        let parent = from.parent_ref::<Global>(shifter);
        match self.timer.get_ratio_easing::<EaseOutBounce>() {
            Some(ratio) => {
                let ratio = ratio as f32;
                to.rectangle_container
                    .render(frame, &Default::default(), ((1. - ratio) * 2., 0.));
                to.texture_container
                    .render(frame, &Default::default(), ((1. - ratio) * 2., 0.));
                from.circle_container.render(
                    frame,
                    &Default::default(),
                    ((1. - ratio) * 2. - 2.0, 0.),
                    parent.resolution,
                );
            }
            _ => {}
        }
    }
}

impl<T> Parent<T> for Global where T: State {}

fn main() {
    let events_loop = glutin::EventsLoop::new();
    let window_size = glutin::dpi::LogicalSize::new(640f64, 640f64);
    let window = glutin::WindowBuilder::new()
        .with_dimensions(window_size)
        .with_title("GLWindow");
    let ctx = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, ctx, &events_loop).unwrap();

    let mut rectangle_container = ShapeContainer::<Rectangle>::new(&display);
    rectangle_container.push(Rectangle {
        pos: [-0.3, 0.0],
        width: 0.2,
        height: 0.5,
        color: [0.1, 0.2, 0.1],
        angle: std::f32::consts::PI / 3.0,
    });

    let mut texture_container = ShapeContainer::<Texture>::new(&display);

    texture_container.push(Texture {
        pos: [0.3, 0.0],
        width: 0.4,
        height: 0.4,
        angle: std::f32::consts::PI / 3.0,
        tex: texture!(&display, "../static/PNG.png", image::PNG),
    });

    let init_state: InitState = InitState {
        counter: 0,
        rectangle_container,
        texture_container,
    };

    let mut circle_container = ShapeContainer::<Circle>::new(&display);
    circle_container.push(Circle {
        pos: [0.0, 0.0],
        r: 0.5,
        color: [0.0, 0.4, 0.4],
    });

    let second_state: SecondState = SecondState {
        counter: 0,
        circle_container,
    };

    let global = Global {
        _timer: Timer::from_millis(std::u64::MAX),
        resolution: {
            let (w, h) = display.get_framebuffer_dimensions();
            (w as i32, h as i32)
        },
    };

    let init_to_second = InitToSecond {
        timer: Timer::from_millis(500),
        transition: Transition::new(),
    };

    let second_to_init = SecondToInit {
        timer: Timer::from_millis(500),
        transition: Transition::new(),
    };

    let mut world = World::new(events_loop, display, init_state)
        .register_state(second_state)
        .register_state(global)
        .register_transition::<InitState, SecondState, _>(init_to_second)
        .register_transition::<SecondState, InitState, _>(second_to_init)
        .finalize();

    world.run();
}
