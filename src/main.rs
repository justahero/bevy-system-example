#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

/// Reference: https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch/blob/main/src/chapter2/src/more_params.rs
mod app;

use app::{App, AppContext};
use std::{any::TypeId, marker::PhantomData, ops::Deref};

struct FunctionSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub trait System {
    fn call(&mut self, context: &mut AppContext);
}

trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl<F: FnMut()> System for FunctionSystem<(), F> {
    fn call(&mut self, _context: &mut AppContext) {
        (self.f)()
    }
}

impl<F: FnMut()> IntoSystem<()> for F
where
    for<'a, 'b> &'a mut F: FnMut(),
{
    type System = FunctionSystem<(), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}

impl<F, T1: SystemParam> System for FunctionSystem<(T1,), F>
where
    for<'a, 'b> &'a mut F: FnMut(T1) + FnMut(<T1 as SystemParam>::Item<'b>),
{
    fn call(&mut self, resources: &mut AppContext) {
        fn call_inner<T1>(mut f: impl FnMut(T1), t1: T1) {
            f(t1);
        }
        call_inner(&mut self.f, T1::retrieve(resources));
    }
}

impl<F, T1: SystemParam> IntoSystem<(T1,)> for F
where
    for<'a, 'b> &'a mut F: FnMut(T1) + FnMut(<T1 as SystemParam>::Item<'b>),
{
    type System = FunctionSystem<(T1,), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}

trait SystemParam {
    type Item<'new>;

    fn retrieve<'r>(context: &'r mut AppContext) -> Self::Item<'r>;
}

struct Res<'a, T: 'static> {
    value: &'a T,
}

impl<'a, T: 'static> Res<'a, T> {
    pub fn inner(&self) -> &'a T {
        self.value
    }
}

impl<T: 'static> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'res, T: 'static> SystemParam for Res<'res, T> {
    type Item<'new> = Res<'new, T>;

    fn retrieve<'r>(context: &'r mut AppContext) -> Self::Item<'r> {
        let value = context
            .resources
            .get(&TypeId::of::<T>())
            .unwrap()
            .downcast_ref()
            .unwrap();
        Res { value }
    }
}

fn foo(number: Res<i32>) {
    println!("Value is {0}", *number);
}

fn main() {
    let mut app = App::new();
    app.add_system(foo);
    app.add_resource(42i32);
    app.run();
    app.run();
}
