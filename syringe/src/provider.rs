use std::marker::PhantomData;

use once_cell::sync::OnceCell;

pub trait Provider<T> {}

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

impl<T> Provider<T> for Transient<T> {}

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

impl<T> Provider<T> for Singleton<T> {}

#[derive(Debug)]
pub struct Instance<T>(T);
impl<T> Instance<T> {
    pub fn new(instance: T) -> Self {
        Self(instance)
    }
}

impl<T> Provider<T> for Instance<T> {}
