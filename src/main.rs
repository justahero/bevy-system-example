#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

/// Reference: https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch/blob/main/src/chapter2/src/more_params.rs
mod app;
mod param;
mod system;

use app::{App, AppContext, CreateWindowHandler, Surface, render};
use param::{Res, State};
use std::{any::TypeId, marker::PhantomData, ops::Deref};

struct MyOne {}

impl CreateWindowHandler for MyOne {
    fn create(surface: &Surface) -> Self {
        MyOne {}
    }
}

fn foo(surface: Res<Surface>, one: State<MyOne>) {
    println!("Surface bar called");
}

fn main() {
    let mut app = App::default()
        .window::<MyOne>(render(foo))
        .run();
}
