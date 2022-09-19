use frunk::hlist::Selector;
use std::{rc::Rc, sync::Arc};

use crate::module::Module;

pub trait SelectProvider<'this, Provider, Index> {
    fn select(&'this self) -> Provider;
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
