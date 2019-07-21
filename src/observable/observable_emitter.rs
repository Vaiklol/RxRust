pub trait ObservableEmitter<S, F>
    where F: FnOnce(S) {
    fn next(&mut self, next: S);
}