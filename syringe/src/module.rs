use frunk::hlist::HList;
use frunk::{HCons, HNil};
use std::rc::Rc;
use std::sync::Arc;

use crate::inject::Inject;
use crate::provider::Provider;

/// `Module` contains all dependencies than can be injected and is used to resolve a struct that implements the `Inject`
/// trait. It contains a list of providers, which determine how individual dependencies are initialized
/// and how they are retrieved when initializing other dependencies.
///
/// There are two primary methods for interacting with a `Module`: `Module::add()` and
/// `Resolver::resolve()`. They add dependencies to a `Module` and resolve structs by injecting
/// dependencies contained within the `Module`, respecitvely.
///
/// If a dependency has not be added to a module, and thus has not been made injectable, a compile-time error
/// will be emitted when attempting to call `Resolver::resolve()`.
#[derive(Debug)]
pub struct Module<ParentModule, Providers> {
    pub(crate) parent_module: ParentModule,
    pub(crate) providers: Providers,
}

impl Module<(), HNil> {
    /// Create an empty instance of `ServiceProvider`
    pub fn new() -> Self {
        Module {
            parent_module: (),
            providers: HNil,
        }
    }
}

impl Default for Module<(), HNil> {
    fn default() -> Self {
        Self::new()
    }
}

impl<ParentModule, Providers> Module<ParentModule, Providers> {
    pub fn fork(&self) -> Module<&Self, HNil> {
        Module {
            parent_module: self,
            providers: HNil,
        }
    }

    pub fn fork_rc(self: &Rc<Self>) -> Module<Rc<Self>, HNil> {
        Module {
            parent_module: self.clone(),
            providers: HNil,
        }
    }

    /// Forking `ServiceProvider` creates a new `ServiceProvider` with reference to the parent.
    /// `resolve` method on forked `ServiceProvider` will find dependencies form self and parent.
    pub fn fork_arc(self: &Arc<Self>) -> Module<Arc<Self>, HNil> {
        Module {
            parent_module: self.clone(),
            providers: HNil,
        }
    }
}

impl<ParentModule, Providers: HList> Module<ParentModule, Providers> {
    pub fn with_provider<'provider, Dependencies, T, P>(
        self,
        provider: P,
    ) -> Module<ParentModule, HCons<P, Providers>>
    where
        T: Inject<Dependencies>,
        P: Provider<'provider, T, Dependencies> + 'provider,
    {
        let Module {
            parent_module,
            providers,
        } = self;
        Module {
            parent_module,
            providers: providers.prepend(provider),
        }
    }
}
