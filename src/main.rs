#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

/// Reference: https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch/blob/main/src/chapter2/src/more_params.rs
mod app;
mod param;
mod system;

use app::{App, AppContext, CreateWindowHandler, Surface, render};
use param::Res;
use std::{any::TypeId, marker::PhantomData, ops::Deref};

struct MyOne {}

impl CreateWindowHandler for MyOne {
    fn create(surface: &Surface) -> Self {
        MyOne {}
    }
}

fn foo(number: Res<i32>) {
    println!("Value is {0}", *number);
}

fn bar(surface: Res<Surface>) {
    println!("Surface bar called");
}

fn baz(surface: &Surface) {
    println!("Surface baz called");
}

fn main() {
    let mut app = App::default()
        .window::<MyOne>(render(foo))
        // app.window<()>(bar);
        // app.window<()>(baz);
        .run();
}
