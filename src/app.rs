use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
};

use crate::{
    param::IntoSystemParam,
    system::{IntoSystem, System},
};

pub type StoredSystem = Box<dyn System>;

pub struct Surface {}

impl IntoSystemParam for Surface {
    type Item<'new> = Self;

    fn convert<'r>(context: &'r WindowContext) -> &'r RefCell<Self::Item<'r>> {
        &context.surface
    }
}

pub trait CreateWindowHandler {
    fn create(surface: &Surface) -> Self
    where
        Self: Sized;
}

impl CreateWindowHandler for () {
    fn create(_surface: &Surface) -> Self {
        ()
    }
}

pub struct WindowHandlers {
    render: StoredSystem,
}

impl WindowHandlers {
    pub fn render<I, S, H>(handler: H) -> Self
    where
        I: 'static,
        S: System + 'static,
        H: IntoSystem<I, System = S>,
    {
        Self {
            render: Box::new(handler.into_system()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Title(pub String);

impl Title {
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }
}

impl IntoSystemParam for Title {
    type Item<'new> = Self;

    fn convert<'r>(context: &'r WindowContext) -> &'r RefCell<Self::Item<'r>> {
        &context.title
    }
}

pub struct WindowContext {
    /// The window title.
    title: RefCell<Title>,
    /// The state instance associated with the window
    state: RefCell<Box<dyn Any>>,
    /// The associated surface to "render" into.
    surface: RefCell<Surface>,
}

impl WindowContext {
    pub fn new(state: Box<dyn Any>) -> Self {
        Self {
            title: RefCell::new(Title::new("Window")),
            state: RefCell::new(state),
            surface: RefCell::new(Surface {}),
        }
    }

    /// Returns the associated state object
    pub fn state<'res>(&self) -> &RefCell<Box<dyn Any>> {
        &self.state
    }
}

pub fn render<I, S, H>(handler: H) -> WindowHandlers
where
    I: 'static,
    S: System + 'static,
    H: IntoSystem<I, System = S>,
{
    WindowHandlers::render(handler)
}

/// Collects all application wide fields and windows.
pub struct AppContext {
    /// The list of all windows
    windows: HashMap<TypeId, (WindowContext, WindowHandlers)>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    fn register(&mut self, state: Box<dyn Any>, handlers: WindowHandlers) {
        let state_type_id = (&*state).type_id();
        let context = WindowContext::new(state);
        self.windows.insert(state_type_id, (context, handlers));
    }
}

type WindowCreateFn = Box<dyn for<'window> Fn(&Surface) -> Box<dyn Any>>;

pub struct App {
    pub windows: Vec<(WindowCreateFn, WindowHandlers)>,
    pub context: AppContext,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        App {
            windows: Vec::new(),
            context: AppContext::new(),
        }
    }

    pub fn window<H>(mut self, handlers: WindowHandlers) -> Self
    where
        H: CreateWindowHandler + 'static,
    {
        let window_create_fn = Box::new(|surface: &Surface| {
            let state = H::create(surface);
            Box::new(state) as Box<dyn Any>
        });
        self.windows.push((window_create_fn, handlers));
        self
    }

    /// Run and consume the app.
    pub fn run(mut self) {
        // Create all windows with their state.
        for (create_fn, handlers) in self.windows.into_iter() {
            let surface = Surface {};
            let state = create_fn(&surface);
            self.context.register(state, handlers);
        }

        // Call all render functions for each window.
        for (_state_type_id, (context, handlers)) in self.context.windows.iter_mut() {
            handlers.render.call(context);
        }
    }
}
