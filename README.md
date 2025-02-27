# bevy-system-example

This repository contains a showcase project on how to replicate [`bevy`]'s internal system or similar systems such as extractors / handlers as implemented in [`axum`] or [`actix-web`]. It builds upon the excellent article [Dependency Injection like Bevy Engine from Scratch](https://promethia-27.github.io/dependency_injection_like_bevy_from_scratch/) with additional functionalities.

The main idea and motiviation was to evaluate how to achieve similar functionality as `bevy` does

A short summary

* multiple separate `Window` instances with their own set of system handlers can be created in `App`
* each `Window` has their own associated state object that can be referenced in their handlers via `State` reference (named after `axum`'s extractor)
* uses `RefCell` internally to support interior mutability
* instead of using a resources container, e.g. a `TypeMap`, separate sources can be fetched via `IntoSystemParam` trait from another source
* the `SystemParam` implementation for `State` contains runtime checks to provide better diagnostics for the user, for example when the same object is referenced multiple times or the target type does not match the state object
* program state(s) can implement the `CreateWindowHandler` trait to generate the window associated state instance
* convenience function `render` bundles the set of handler functions (currently only one), similar to `axum`'s `MethodRouter`
* `App::run` consumes the instance, generates all windows in the first step, then calls the `render` handler for each in a second step
* the `WindowContext` holds a few custom fields, instead of using a generic resources container, to extract data from

[`actix-web`]: https://actix.rs/
[`axum`]: https://github.com/tokio-rs/axum
[`bevy`]: https://bevyengine.org/
