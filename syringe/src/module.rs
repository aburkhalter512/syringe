use frunk::hlist::HList;
use frunk::{HCons, HNil};
use std::rc::Rc;
use std::sync::Arc;

use crate::inject::Inject;
use crate::provider::{InstanceProvider, Provider, SingletonProvider, TransientProvider};

/// `ServiceProvider` struct is used as an IoC-container in which you declare your dependencies.
///
/// Algorithm for working in `ServiceProvider` is:
/// 1. Create an empty by `ServiceProvider::new` function.
/// 2. Declare your dependencies using `add_*` methods (more about theirs read below).
/// 3. Fork `ServiceProvider` when you need working with scoped sessions (like when you processing web request).
/// 4. Get needed dependencies from container using `Resolver::resolve` trait.
///
/// If you do not register all of needed dependencies, then compiler do not compile your code. If error
/// puts you into a stupor, read our [manual] about how read errors.
///
/// [manual]: https://github.com/p0lunin/teloc/blob/master/HOW-TO-READ-ERRORS.md
///
/// Example of usage `ServiceProvider`:
/// ```
/// use std::rc::Rc;
/// use teloc::*;
///
/// struct ConstService {
///     number: Rc<i32>,
/// }
///
/// #[inject]
/// impl ConstService {
///     pub fn new(number: Rc<i32>) -> Self {
///         ConstService { number }
///     }
/// }
///
/// #[derive(Dependency)]
/// struct Controller {
///     number_service: ConstService,
/// }
///
/// let container = ServiceProvider::new()
///     .add_transient::<ConstService>()
///     .add_transient::<Controller>();
/// let scope = container.fork().add_instance(Rc::new(10));
/// let controller: Controller = scope.resolve();
/// assert_eq!(*controller.number_service.number, 10);
/// ```
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
    pub fn add<Dependencies, T: Inject<Dependencies>, P: Provider<T>>(
        self,
        provider: P,
    ) -> Module<ParentModule, HCons<P, Providers>> {
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
