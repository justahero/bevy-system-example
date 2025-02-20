use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{IntoSystem, System};

pub type StoredSystem = Box<dyn System>;

pub struct App {
    pub systems: Vec<StoredSystem>,
    pub resources: HashMap<TypeId, Box<dyn Any>>,
}

impl App {
    pub fn new() -> Self {
        App {
            systems: Vec::new(),
            resources: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.call(&mut self.resources);
        }
    }

    pub fn add_system<I, S: System + 'static>(&mut self, system: impl IntoSystem<I, System = S>) {
        self.systems.push(Box::new(system.into_system()));
    }

    pub fn add_resource<R: 'static>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }
}
