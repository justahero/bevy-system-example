use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    ops::Deref,
};

use crate::app::WindowContext;

pub struct State<'a, T> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> State<'a, T> {
    pub fn new(value: RefMut<'a, Box<dyn Any>>) -> Self {
        Self {
            value,
            _marker: PhantomData::default(),
        }
    }
}

impl<T: 'static> Deref for State<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.downcast_ref().expect("Failed to cast state.")
    }
}

pub struct Res<'a, T: IntoSystemParam> {
    value: Ref<'a, T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IntoSystemParam> Res<'a, T> {
    pub fn new(value: Ref<'a, T>) -> Self {
        Self {
            value,
            _marker: PhantomData::default(),
        }
    }
}

pub struct ResMut<'a, T: IntoSystemParam> {
    value: RefMut<'a, T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: IntoSystemParam> ResMut<'a, T> {
    pub fn new(value: RefMut<'a, T>) -> Self {
        Self {
            value,
            _marker: PhantomData::default(),
        }
    }
}

/// Trait to extract some `Item` from the `WindowContext` for some implementation, e.g. Surface.
pub trait SystemParam {
    /// Associated type `Item` is declared here to allow to re-assign the lifetime of `Self`.
    type Item<'new>;

    fn extract<'r>(context: &'r WindowContext) -> Self::Item<'r>;
}

pub trait IntoSystemParam: 'static {
    type Item<'new>;

    fn convert<'r>(context: &'r WindowContext) -> &'r RefCell<Self::Item<'r>>;
}

impl<'res, T> SystemParam for Res<'res, T>
where
    T: for<'a> IntoSystemParam<Item<'a> = T>,
{
    type Item<'new> = Res<'new, T>;

    fn extract<'r>(context: &'r WindowContext) -> Self::Item<'r> {
        Res::new(T::convert(context).borrow())
    }
}

impl<'res, T: 'static> SystemParam for ResMut<'res, T>
where
    T: for<'a> IntoSystemParam<Item<'a> = T>,
{
    type Item<'new> = ResMut<'new, T>;
    
    fn extract<'r>(context: &'r WindowContext) -> Self::Item<'r> {
        ResMut::new(T::convert(context).borrow_mut())
    }
}

impl<'res, T: 'static> SystemParam for State<'res, T> {
    type Item<'new> = State<'new, T>;

    fn extract<'r>(context: &'r WindowContext) -> Self::Item<'r> {
        State::new(context.state().borrow_mut())
    }
}
