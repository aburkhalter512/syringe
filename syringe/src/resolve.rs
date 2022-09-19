use frunk::prelude::*;
use frunk::HCons;
use frunk::HNil;

use crate::provider::Singleton;
use crate::provider::{Instance, Provider, Transient};
use crate::select_provider::SelectProvider;

pub trait Resolver<'this, Provider, T, Infer> {
    fn resolve(&'this self) -> T;
}

impl<'module, 'provider, Module, T, Dependencies, Index, Infer>
    Resolver<'module, Transient<T>, T, (Dependencies, Index, Infer)> for Module
where
    Module: SelectProvider<'module, &'provider Transient<T>, Index>
        + ResolveDependencies<'module, Dependencies, Infer>,
    Transient<T>: Provider<'provider, T, Dependencies>,
    T: 'provider,
{
    fn resolve(&'module self) -> T {
        let dependencies = self.resolve_dependencies();
        self.select().provide(dependencies)
    }
}

impl<'module, 'provider, Module, T, Dependencies, Index, Infer>
    Resolver<'module, Singleton<T>, &'provider T, (Dependencies, Index, Infer)> for Module
where
    Module: SelectProvider<'module, &'provider Singleton<T>, Index>
        + ResolveDependencies<'module, Dependencies, Infer>,
    Singleton<T>: Provider<'provider, &'provider T, Dependencies>,
    T: 'provider,
{
    fn resolve(&'module self) -> &'provider T {
        let dependencies = self.resolve_dependencies();
        self.select().provide(dependencies)
    }
}

impl<'module, 'provider, Module, T, Index> Resolver<'module, Instance<T>, &'provider T, Index>
    for Module
where
    Module: SelectProvider<'module, &'provider Instance<T>, Index>,
    Instance<T>: Provider<'provider, &'provider T, HNil>,
{
    fn resolve(&'module self) -> &'provider T {
        self.select().provide(HNil)
    }
}

pub trait ResolveDependencies<'module, Dependencies, Infer> {
    fn resolve_dependencies(&'module self) -> Dependencies;
}

impl<'module, Dependency, DependenciesRest, Provider, Infer, InferRest, Module>
    ResolveDependencies<
        'module,
        HCons<Dependency, DependenciesRest>,
        HCons<(Provider, Infer), InferRest>,
    > for Module
where
    DependenciesRest: HList,
    Module: Resolver<'module, Provider, Dependency, Infer>
        + ResolveDependencies<'module, DependenciesRest, InferRest>,
{
    fn resolve_dependencies(&'module self) -> HCons<Dependency, DependenciesRest> {
        ResolveDependencies::<DependenciesRest, InferRest>::resolve_dependencies(self)
            .prepend(self.resolve())
    }
}

impl<'module, Module> ResolveDependencies<'module, HNil, HNil> for Module {
    fn resolve_dependencies(&'module self) -> HNil {
        HNil
    }
}
