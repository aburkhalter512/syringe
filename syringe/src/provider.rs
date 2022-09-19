use std::marker::PhantomData;

use frunk::HNil;
use once_cell::sync::OnceCell;

use crate::inject::Inject;

pub trait Provider<'this, T, Dependencies> {
    fn provide(&'this self, dependencies: Dependencies) -> T;
}

pub struct Transient<T>(PhantomData<T>);
impl<T> Transient<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Transient<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'this, T, Dependencies> Provider<'this, T, Dependencies> for Transient<T>
where
    T: Inject<Dependencies>,
{
    fn provide(&'this self, dependencies: Dependencies) -> T {
        T::inject(dependencies)
    }
}

#[derive(Debug)]
pub struct Singleton<T>(OnceCell<T>);
impl<T> Singleton<T> {
    pub fn new() -> Self {
        Self(OnceCell::new())
    }
}

impl<T> Default for Singleton<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'this, T, Dependencies> Provider<'this, &'this T, Dependencies> for Singleton<T>
where
    T: Inject<Dependencies>,
{
    fn provide(&'this self, dependencies: Dependencies) -> &'this T {
        self.0.get_or_init(|| T::inject(dependencies))
    }
}

#[derive(Debug)]
pub struct Instance<T>(T);
impl<T> Instance<T> {
    pub fn new(instance: T) -> Self {
        Self(instance)
    }
}

impl<'this, T> Provider<'this, &'this T, HNil> for Instance<T> {
    fn provide(&'this self, _: HNil) -> &'this T {
        &self.0
    }
}
