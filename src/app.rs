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

pub struct WindowContext {
    surface: Surface,
    render: StoredSystem,
}

impl WindowContext {
    pub fn render<I, S, H>(handler: H) -> Self
    where
        I: 'static,
        S: System + 'static,
        H: IntoSystem<I, System = S>,
    {
        Self {
            surface: Surface {},
            render: Box::new(handler.into_system()),
        }
    }
}

pub fn render<I, S, H>(handler: H) -> WindowContext
where
    I: 'static,
    S: System + 'static,
    H: IntoSystem<I, System = S>,
{
    WindowContext::render(handler)
}

pub struct AppContext {
    windows: HashMap<TypeId, (Box<dyn Any>, WindowContext)>,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    fn register(&mut self, instance: Box<dyn Any>, context: WindowContext) {
        let state_type_id = (&*instance).type_id();
        self.windows.insert(state_type_id, (instance, context));
    }
}

type WindowCreateFn = Box<dyn for<'window> Fn(&Surface) -> Box<dyn Any>>;

pub struct App {
    pub windows: Vec<(WindowCreateFn, WindowContext)>,
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

    pub fn window<H>(mut self, context: WindowContext) -> Self
    where
    H: CreateWindowHandler + 'static
    {
        let window_create_fn = Box::new(|surface: &Surface| {
            let state = H::create(surface);
            Box::new(state) as Box<dyn Any>
        });
        self.windows.push((window_create_fn, context));
        self
    }

    pub fn run(&mut self) {
        todo!()
    }
}
