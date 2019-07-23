pub trait ObservableEmitter<R, T, P> {
    fn next(&mut self, next: R);
}