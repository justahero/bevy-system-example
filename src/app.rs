use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    param::{IntoSystemParam, SystemParam},
    system::{IntoSystem, System},
};

pub type TypeMap = HashMap<TypeId, Box<dyn Any>>;

pub type StoredSystem = Box<dyn System>;

pub struct Surface {}

impl IntoSystemParam for Surface {
    type Item<'new> = Self;

    fn convert<'r>(context: &'r mut WindowContext) -> &'r Self::Item<'r> {
        &context.surface
    }
}

impl<'res> SystemParam for &'res Surface {
    type Item<'new> = &'new Surface;

    fn extract<'r>(context: &'r mut WindowContext) -> Self::Item<'r> {
        &context.surface
    }
}

pub trait CreateWindowHandler {
    fn create(surface: &Surface) -> Self
    where
        Self: Sized;
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

pub struct WindowContext {
    /// The state instance associated with the window
    state: Box<dyn Any>,
    /// The associated surface to "render" into.
    surface: Surface,
}

impl WindowContext {
    pub fn new(state: Box<dyn Any>) -> Self {
        Self {
            state,
            surface: Surface {},
        }
    }

    pub fn state_type_id(&self) -> TypeId {
        (&*self.state).type_id()
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
        let context = WindowContext::new(state);
        let state_type_id = context.state_type_id();

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
