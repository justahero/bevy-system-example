use std::{
    any::Any,
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::app::WindowContext;

/// Window related state accessor.
pub struct State<'a, T> {
    value: RefMut<'a, Box<dyn Any>>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> State<'a, T> {
    pub fn new(value: RefMut<'a, Box<dyn Any>>) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T: 'static> Deref for State<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.downcast_ref().expect("Failed to cast state.")
    }
}

impl<T: 'static> DerefMut for State<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.downcast_mut().expect("Failed to cast object")
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
            _marker: PhantomData,
        }
    }
}

impl<T: IntoSystemParam> Deref for Res<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

pub struct ResMut<'a, T: IntoSystemParam + Any> {
    value: RefMut<'a, T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: IntoSystemParam> ResMut<'a, T> {
    pub fn new(value: RefMut<'a, T>) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<T: IntoSystemParam> Deref for ResMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.value.deref()
    }
}

impl<T: IntoSystemParam> DerefMut for ResMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value.deref_mut()
    }
}

/// Trait to extract some `Item` from the `WindowContext` for some implementation, e.g. Surface.
pub trait SystemParam {
    /// Associated type `Item` is declared here to allow to re-assign the lifetime of `Self`.
    type Item<'new>;

    fn extract(context: &WindowContext) -> Self::Item<'_>;
}

pub trait IntoSystemParam: 'static {
    type Item<'new>;

    fn convert(context: &WindowContext) -> &RefCell<Self::Item<'_>>;
}

impl<T> SystemParam for Res<'_, T>
where
    T: for<'a> IntoSystemParam<Item<'a> = T>,
{
    type Item<'new> = Res<'new, T>;

    fn extract(context: &WindowContext) -> Self::Item<'_> {
        Res::new(T::convert(context).borrow())
    }
}

impl<T: 'static> SystemParam for ResMut<'_, T>
where
    T: for<'a> IntoSystemParam<Item<'a> = T>,
{
    type Item<'new> = ResMut<'new, T>;

    fn extract(context: &WindowContext) -> Self::Item<'_> {
        ResMut::new(T::convert(context).borrow_mut())
    }
}

impl<T: 'static> SystemParam for State<'_, T> {
    type Item<'new> = State<'new, T>;

    fn extract(context: &WindowContext) -> Self::Item<'_> {
        let expected_type_name = core::any::type_name::<T>();

        // Check that the State object is not already borrowed mutably
        if context.state().try_borrow_mut().is_err() {
            panic!(
                "State '{}' is already exclusively (mutably) borrowed!",
                expected_type_name
            );
        }

        // Check that the internal state can actually be casted into the target type T.
        {
            let borrow = context.state().borrow();
            match borrow.downcast_ref::<T>() {
                Some(_) => {}
                None => {
                    panic!("Failed to cast state to '{}'", expected_type_name);
                }
            }
        }

        State::new(context.state().borrow_mut())
    }
}
