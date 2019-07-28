pub struct ObservableMap<S, F>
    where F: FnOnce(S) {
    transform: F,
    source: S,
}