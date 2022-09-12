use frunk::hlist::Selector;
use std::{rc::Rc, sync::Arc};

use crate::module::Module;

/// Borrow containers from a ServiceProvider.
///
/// ```
/// # use teloc::{Dependency, Resolver, ServiceProvider};
/// # use teloc::dev::*;
/// let cont: &InstanceContainer<i32> =
///     ServiceProvider::new().add_instance(10i32).get();
/// ```
///
/// Can borrow containers from a forked ServiceProvider that would outlive the fork
///
/// ```
/// # use teloc::*;
/// # use teloc::dev::*;
/// let provider = ServiceProvider::new().add_instance(10i32);
/// let invalid_singleton: &InstanceContainer<i32> = {
///     let provider = provider.fork();
///     provider.get()
/// };
///```
///
/// Cannot borrow containers from a ServiceProvider that would outlive the ServiceProvider
///
/// ```compile_fail
/// # use teloc::*;
/// # use teloc::dev::*;
/// let invalid_singleton: &InstanceContainer<i32> = {
///     let provider = ServiceProvider::new().add_instance(10i32);
///     provider.get()
/// };
///```
pub trait SelectProvider<'a, Provider, Index> {
    fn select(&'a self) -> Provider;
}

pub struct SelfIndex<InnerIndex>(InnerIndex);
pub struct ParentIndex<InnerIndex>(InnerIndex);

impl<'this, ParentModule, Providers, Provider, Index>
    SelectProvider<'this, &'this Provider, SelfIndex<Index>> for Module<ParentModule, Providers>
where
    Providers: Selector<Provider, Index>,
{
    fn select(&'this self) -> &'this Provider {
        self.providers.get()
    }
}

impl<'this, 'parent, 'provider, ParentModule, Providers, Provider, Index>
    SelectProvider<'this, &'provider Provider, ParentIndex<Index>>
    for Module<&'parent ParentModule, Providers>
where
    ParentModule: SelectProvider<'parent, &'provider Provider, Index>,
{
    fn select(&'this self) -> &'provider Provider {
        self.parent_module.select()
    }
}

impl<'this, 'provider, ParentModule, Providers, Provider, Index>
    SelectProvider<'this, &'provider Provider, ParentIndex<Index>>
    for Module<Rc<ParentModule>, Providers>
where
    ParentModule: SelectProvider<'this, &'provider Provider, Index>,
{
    fn select(&'this self) -> &'provider Provider {
        self.parent_module.select()
    }
}

impl<'this, 'provider, ParentModule, Providers, Provider, Index>
    SelectProvider<'this, &'provider Provider, ParentIndex<Index>>
    for Module<Arc<ParentModule>, Providers>
where
    ParentModule: SelectProvider<'this, &'provider Provider, Index>,
{
    fn select(&'this self) -> &'provider Provider {
        self.parent_module.select()
    }
}
