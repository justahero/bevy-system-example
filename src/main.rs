#![allow(unused_variables)]
#![allow(dead_code)]

/// Reference: https://github.com/PROMETHIA-27/dependency_injection_like_bevy_from_scratch/blob/main/src/chapter2/src/more_params.rs
mod app;
mod param;
mod system;

use app::{App, CreateWindowHandler, Surface, Title, render};
use param::{Res, ResMut, State};

#[derive(Debug)]
struct MyOne {
    title: String,
}

impl CreateWindowHandler for MyOne {
    fn create(_surface: &Surface) -> Self {
        MyOne {
            title: "Hello World".to_string(),
        }
    }
}

#[derive(Debug)]
struct MyTwo(pub i32);

impl CreateWindowHandler for MyTwo {
    fn create(_surface: &Surface) -> Self {
        MyTwo(42)
    }
}

fn foo(_surface: ResMut<Surface>, one: State<MyOne>, mut title: ResMut<Title>) {
    *title = Title::new("Hello");
    println!("Function foo called with surface and state: {:?}", *one);
}

fn bar(two: State<MyTwo>, title: Res<Title>, mut state: State<MyTwo>) {
    println!(
        "Function bar called with title: {:?} with value: {:?}",
        *title, *two
    );
    *state.0 = 42;
}

fn bla() {
    println!("Function bla called");
}

fn main() {
    App::default()
        .window::<MyOne>(render(foo))
        .window::<MyTwo>(render(bar))
        .window::<()>(render(bla))
        .run();
}
