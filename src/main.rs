#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

/// Reference: https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch/blob/main/src/chapter2/src/more_params.rs
mod app;
mod param;
mod system;

use app::{App, AppContext, CreateWindowHandler, Surface, render};
use param::{Res, ResMut, State};
use std::{any::TypeId, marker::PhantomData, ops::Deref};

#[derive(Debug)]
struct MyOne {
    title: String,
}

impl CreateWindowHandler for MyOne {
    fn create(surface: &Surface) -> Self {
        MyOne {
            title: "Hello World".to_string(),
        }
    }
}

struct MyTwo;

impl CreateWindowHandler for MyTwo {
    fn create(surface: &Surface) -> Self {
        MyTwo {}
    }
}

fn foo(surface: ResMut<Surface>, one: State<MyOne>) {
    println!("Function foo called with surface and state: {:?}", *one);
}

fn bar() {
    println!("Function bar called");
}

fn main() {
    let mut app = App::default()
        .window::<MyOne>(render(foo))
        .window::<MyTwo>(render(bar))
        .window::<()>(render(bar))
        .run();
}
