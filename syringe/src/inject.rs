pub trait Inject<Dependencies> {
    fn inject(deps: Dependencies) -> Self;
}
