pub trait ObservableEmitter<S> {
    fn next(&self, next: S);
}
