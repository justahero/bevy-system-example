/// Reference: https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch/blob/main/src/chapter2/src/more_params.rs
mod app;
mod param;
mod system;

use app::{App, AppContext, CreateWindowHandler, Surface, Title, render};
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

#[derive(Debug)]
struct MyTwo(pub i32);

impl CreateWindowHandler for MyTwo {
    fn create(surface: &Surface) -> Self {
        MyTwo(42)
    }
}

fn foo(surface: ResMut<Surface>, one: State<MyOne>) {
    println!("Function foo called with surface and state: {:?}", *one);
}

fn bar(title: Title, two: State<MyTwo>) {
    println!("Function bar called with title: {:?} with value: {:?}", title, *two);
}

fn main() {
    App::default()
        .window::<MyOne>(render(foo))
        .window::<MyTwo>(render(bar))
        .window::<()>(render(bar))
        .run();
}
