use std::ops::Deref;

use crate::app::AppContext;

pub struct Res<'a, T: IntoSystemParam> {
    value: &'a T,
}

impl<T: IntoSystemParam> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value
    }
}

/// Trait to extract some `Item` from the `AppContext` for some implementation, e.g. Surface.
pub trait SystemParam {
    /// Associated type `Item` is declared here to allow to re-assign the lifetime of `Self`.
    type Item<'new>;

    fn extract<'r>(context: &'r mut AppContext) -> Self::Item<'r>;
}

pub trait IntoSystemParam: 'static {
    type Item<'new>;

    fn convert<'r>(context: &'r mut AppContext) -> &'r Self::Item<'r>;
}

impl<'a, T: IntoSystemParam> Res<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Self { value }
    }

    pub fn inner(&self) -> &'a T {
        self.value
    }
}

impl IntoSystemParam for i32 {
    type Item<'new> = Self;

    fn convert<'r>(context: &'r mut AppContext) -> &'r Self::Item<'r> {
        &0
    }
}

impl<'res, T> SystemParam for Res<'res, T>
where
    T: for<'a> IntoSystemParam<Item<'a> = T>,
{
    type Item<'new> = Res<'new, T>;

    fn extract<'r>(context: &'r mut AppContext) -> Self::Item<'r> {
        Res::new(T::convert(context))
    }
}
