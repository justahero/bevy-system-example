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

    fn convert<'r>(context: &'r mut AppContext) -> &'r Self::Item<'r> {
        &context.surface
    }
}

impl<'res> SystemParam for &'res Surface {
    type Item<'new> = &'new Surface;

    fn extract<'r>(context: &'r mut AppContext) -> Self::Item<'r> {
        &context.surface
    }
}

pub struct AppContext {
    pub surface: Surface,
    pub resources: TypeMap,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            surface: Surface {},
            resources: TypeMap::new(),
        }
    }
}

pub struct App {
    pub systems: Vec<StoredSystem>,
    pub context: AppContext,
}

impl App {
    pub fn new() -> Self {
        App {
            systems: Vec::new(),
            context: AppContext::new(),
        }
    }

    pub fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.call(&mut self.context);
        }
    }

    pub fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
        self.systems.push(Box::new(system.into_system()));
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.context
            .resources
            .insert(TypeId::of::<R>(), Box::new(resource));
    }
}
