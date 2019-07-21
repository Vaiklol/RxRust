pub trait ObservableEmitter<S> {
    fn next(&mut self, next: S);
}
