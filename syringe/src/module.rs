use frunk::hlist::HList;
use frunk::{HCons, HNil};
use std::rc::Rc;
use std::sync::Arc;

use crate::provider::{InstanceProvider, SingletonProvider, TransientProvider};

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
    pub fn add<Provider>(
        self,
        provider: Provider,
    ) -> Module<ParentModule, HCons<Provider, Providers>> {
        let Module {
            parent_module,
            providers,
        } = self;
        Module {
            parent_module,
            providers: providers.prepend(provider),
        }
    }

    /// Add dependency with the `Transient` lifetime. Transient services will be created each time
    /// when it called. Use this lifetime for lightweight stateless services.
    ///
    /// Can be resolved only by ownership.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    /// use uuid::Uuid;
    ///
    /// struct Service { uuid: Uuid }
    /// #[inject]
    /// impl Service {
    ///     fn new() -> Self { Self { uuid: Uuid::new_v4() } }
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_transient::<Service>();
    ///
    /// let s1: Service = sp.resolve();
    /// let s2: Service = sp.resolve();
    ///
    /// assert_ne!(s1.uuid, s2.uuid);
    /// ```
    pub fn add_transient<T>(self) -> Module<ParentModule, HCons<TransientProvider<T>, Providers>> {
        self.add(TransientProvider::<T>::new())
    }

    /// Add dependency with the `Singleton` lifetime. Singleton services will be created only one
    /// time when it will be called first time. It will be same between different calls in parent
    /// and forked `ServiceProvider`
    ///
    /// Can be resolved by reference or by cloning. If you wish to clone this dependency then it
    /// must implement `DependencyClone` trait. For more information see `DependencyClone` trait.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    /// use uuid::Uuid;
    ///
    /// struct Service { uuid: Uuid }
    /// #[inject]
    /// impl Service {
    ///     fn new() -> Self { Self { uuid: Uuid::new_v4() } }
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_singleton::<Service>();
    /// let scope = sp.fork();
    ///
    /// let s1: &Service = sp.resolve();
    /// let s2: &Service = scope.resolve();
    ///
    /// assert_eq!(s1.uuid, s2.uuid);
    /// ```
    ///
    /// Usage with cloning:
    ///
    /// ```
    /// use teloc::*;
    /// use uuid::Uuid;
    /// use std::rc::Rc;
    ///
    /// struct Service { uuid: Uuid }
    /// #[inject]
    /// impl Service {
    ///     fn new() -> Self { Self { uuid: Uuid::new_v4() } }
    /// }
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_singleton::<Rc<Service>>();
    ///
    /// let s1: Rc<Service> = sp.resolve();
    /// let s2: Rc<Service> = sp.resolve();
    ///
    /// assert_eq!(s1.uuid, s2.uuid)
    /// ```
    pub fn add_singleton<T>(self) -> Module<ParentModule, HCons<SingletonProvider<T>, Providers>> {
        self.add(SingletonProvider::<T>::new())
    }

    /// Add anything instance to provider. It likes singleton, but it cannot get dependencies from
    /// the provider. Use it for adding single objects like configs.
    ///
    /// Can be resolved by reference or by cloning. If you wish to clone this dependency then it
    /// must implement `DependencyClone` trait. For more information see `DependencyClone` trait.
    ///
    /// Usage:
    /// ```
    /// use teloc::*;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct Config { token: String, ip: String }
    ///
    /// struct Service<'a> { token: &'a str, ip: &'a str }
    /// #[inject]
    /// impl<'a> Service<'a> {
    ///     fn new(config: &'a Config) -> Self { Self { token: &config.token, ip: &config.ip } }
    /// }
    ///
    /// let config = Config { token: "1234ABCDE".into(), ip: "192.168.0.1".into() };
    ///
    /// let sp = ServiceProvider::new()
    ///     .add_instance(&config)
    ///     .add_transient::<Service>();
    ///
    /// let config_ref: &Config = sp.resolve();
    /// let s: Service = sp.resolve();
    ///
    /// assert_eq!(&config, config_ref);
    /// assert_eq!(&config_ref.token, s.token);
    /// assert_eq!(&config_ref.ip, s.ip);
    /// ```
    pub fn add_instance<T>(
        self,
        data: T,
    ) -> Module<ParentModule, HCons<InstanceProvider<T>, Providers>> {
        self.add(InstanceProvider::<T>::new(data))
    }
}
