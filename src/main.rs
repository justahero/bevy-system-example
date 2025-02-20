#![allow(dead_code)]
#![allow(unused_mut)]

mod scheduler;

use scheduler::Scheduler;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

type TypeMap = HashMap<TypeId, Box<dyn Any>>;

struct FunctionSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub trait System {
    fn call(&mut self, resources: &mut TypeMap);
}

impl<F: FnMut()> System for FunctionSystem<(), F> {
    fn call(&mut self, _resources: &mut TypeMap) {
        (self.f)()
    }
}

impl<F: FnMut(T1), T1: 'static> System for FunctionSystem<(T1,), F> {
    fn call(&mut self, resources: &mut TypeMap) {
        let t1 = *resources
            .remove(&TypeId::of::<T1>())
            .unwrap()
            .downcast::<T1>()
            .unwrap();
        (self.f)(t1)
    }
}
impl<F: FnMut(T1, T2), T1: 'static, T2: 'static> System for FunctionSystem<(T1, T2), F> {
    fn call(&mut self, resources: &mut TypeMap) {
        let t1 = *resources
            .remove(&TypeId::of::<T1>())
            .unwrap()
            .downcast::<T1>()
            .unwrap();
        let t2 = *resources
            .remove(&TypeId::of::<T2>())
            .unwrap()
            .downcast::<T2>()
            .unwrap();
        (self.f)(t1, t2)
    }
}

trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl<F: FnMut()> IntoSystem<()> for F {
    type System = FunctionSystem<(), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}

impl<F: FnMut(T1), T1: 'static> IntoSystem<(T1,)> for F {
    type System = FunctionSystem<(T1,), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}

impl<F: FnMut(T1, T2), T1: 'static, T2: 'static> IntoSystem<(T1, T2)> for F {
    type System = FunctionSystem<(T1, T2), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}

fn foo(value: i32) {
    println!("Value is {value}");
}

fn main() {
    let mut scheduler = Scheduler::new();
    scheduler.add_system(foo);
    scheduler.add_resource(42i32);
    scheduler.run();
}
