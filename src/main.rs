use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

type TypeMap = HashMap<TypeId, Box<dyn Any>>;

pub trait System<Input> {
    fn call(&mut self, resources: &mut TypeMap);
}

impl<F: FnMut()> System<()> for F {
    fn call(&mut self, _resources: &mut TypeMap) {
        (self)()
    }
}

impl<F: FnMut(T1), T1: 'static> System<(T1,)> for F {
    fn call(&mut self, resources: &mut TypeMap) {
        let t1 = *resources.remove(&TypeId::of::<T1>()).unwrap().downcast::<T1>().unwrap();
        (self)(t1)
    }
}
impl<F: FnMut(T1, T2), T1: 'static, T2: 'static> System<(T1,T2)> for F {
    fn call(&mut self, resources: &mut TypeMap) {
        let t1 = *resources.remove(&TypeId::of::<T1>()).unwrap().downcast::<T1>().unwrap();
        let t2 = *resources.remove(&TypeId::of::<T2>()).unwrap().downcast::<T2>().unwrap();
        (self)(t1, t2)
    }
}

trait ErasedSystem {
    fn call(&mut self, resources: &mut TypeMap);
}

impl<S: System<I>, I> ErasedSystem for S {
    fn call(&mut self, resources: &mut TypeMap) {
        <Self as System<I>>::call(self, resources)
    }
}

struct StoredSystem {}

struct Scheduler {
    systems: Vec<StoredSystem>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}

fn main() {
    println!("Hello, world!");
}
