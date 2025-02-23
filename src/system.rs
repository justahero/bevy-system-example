use std::marker::PhantomData;

use crate::{Res, app::WindowContext, param::SystemParam};

pub struct FunctionSystem<Input, F> {
    f: F,
    marker: PhantomData<fn() -> Input>,
}

pub trait System {
    fn call(&mut self, context: &mut WindowContext);
}

pub trait IntoSystem<Input> {
    type System: System;

    fn into_system(self) -> Self::System;
}

impl<F: FnMut()> System for FunctionSystem<(), F> {
    fn call(&mut self, _context: &mut WindowContext) {
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
    fn call(&mut self, context: &mut WindowContext) {
        fn call_inner<T1>(mut f: impl FnMut(T1), t1: T1) {
            f(t1);
        }
        call_inner(&mut self.f, T1::extract(context));
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

impl<F, T1: SystemParam, T2: SystemParam> System for FunctionSystem<(T1, T2), F>
where
    for<'a, 'b> &'a mut F: FnMut(T1, T2) + FnMut(<T1 as SystemParam>::Item<'b>, <T2 as SystemParam>::Item<'b>),
{
    fn call(&mut self, context: &mut WindowContext) {
        fn call_inner<T1, T2>(mut f: impl FnMut(T1, T2), t1: T1, t2: T2) {
            f(t1, t2);
        }

        let t1 = T1::extract(context);
        let t2 = T2::extract(context);
        call_inner(&mut self.f, t1, t2);
    }
}

impl<F, T1: SystemParam, T2: SystemParam> IntoSystem<(T1, T2)> for F
where
    for<'a, 'b> &'a mut F: FnMut(T1, T2) + FnMut(<T1 as SystemParam>::Item<'b>, <T2 as SystemParam>::Item<'b>),
{
    type System = FunctionSystem<(T1, T2), Self>;

    fn into_system(self) -> Self::System {
        FunctionSystem {
            f: self,
            marker: Default::default(),
        }
    }
}
