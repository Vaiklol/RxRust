pub trait ObservableEmitter<R, T, P> {
    fn next(&mut self, next: R);

    fn error(&mut self, error: P);
}