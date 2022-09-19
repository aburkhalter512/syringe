use frunk::HNil;

use crate::provider::{Instance, Provider};
use crate::select_provider::SelectProvider;

pub trait Resolver<'this, Provider, T, Infer> {
    fn resolve(&'this self) -> T;
}

impl<'module, 'provider, Module, T, Infer> Resolver<'module, Instance<T>, &'provider T, Infer>
    for Module
where
    Module: SelectProvider<'module, &'provider Instance<T>, Infer>,
    Instance<T>: Provider<'provider, &'provider T, HNil>,
{
    fn resolve(&'module self) -> &'provider T {
        self.select().provide(HNil)
    }
}
