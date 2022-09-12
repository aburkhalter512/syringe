use std::marker::PhantomData;

use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct TransientProvider<T>(PhantomData<T>);
impl<T> TransientProvider<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for TransientProvider<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SingletonProvider<T>(OnceCell<T>);
impl<T> SingletonProvider<T> {
    pub fn new() -> Self {
        Self(OnceCell::new())
    }
}

impl<T> Default for SingletonProvider<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct InstanceProvider<T>(T);
impl<T> InstanceProvider<T> {
    pub fn new(instance: T) -> Self {
        Self(instance)
    }
}
