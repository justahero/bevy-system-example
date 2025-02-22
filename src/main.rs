#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

/// Reference: https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch/blob/main/src/chapter2/src/more_params.rs
mod app;

use app::{App, AppContext, Surface};
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
        call_inner(&mut self.f, T1::extract(resources));
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

impl SystemParam for Surface {
    type Item<'new> = &'new Self;

    fn extract<'r>(context: &'r mut AppContext) -> Self::Item<'r> {
        &context.surface
    }
}

/// Trait to extract some `Item` from the `AppContext` for some implementation, e.g. Surface.
trait SystemParam {
    /// Associated type `Item` is declared here to allow to re-assign the lifetime of `Self`.
    type Item<'new>;

    fn extract<'r>(context: &'r mut AppContext) -> Self::Item<'r>;
}

trait Resource: 'static {
    type Item<'new>;

    fn extract<'r>(context: &'r mut AppContext) -> &'r Self::Item<'r>;
}

struct Res<'a, T: Resource> {
    value: &'a T,
}

impl<'a, T: Resource> Res<'a, T> {
    pub fn inner(&self) -> &'a T {
        self.value
    }
}

impl<T: Resource> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl Resource for i32 {
    type Item<'new> = i32;

    fn extract<'r>(context: &'r mut AppContext) -> &'r Self::Item<'r> {
        &0
    }
}

impl Resource for Surface {
    type Item<'new> = Self;

    fn extract<'r>(context: &'r mut AppContext) -> &'r Self::Item<'r> {
        &context.surface
    }
}

impl<'res, T> SystemParam for Res<'res, T>
where
    T: for<'a> Resource<Item<'a> = T>,
{
    type Item<'new> = Res<'new, T>;

    fn extract<'r>(context: &'r mut AppContext) -> Self::Item<'r> {
        let value = T::extract(context);
        Res { value: &value }
    }
}

fn foo(number: Res<i32>) {
    println!("Value is {0}", *number);
}

fn bar(surface: Res<Surface>) {
    println!("Surface called");
}

fn main() {
    let mut app = App::new();
    app.add_system(foo);
    app.add_system(bar);
    app.run();
    app.run();
}
