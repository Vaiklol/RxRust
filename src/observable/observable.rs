use crate::observable::observable_emitter::ObservableEmitter;
use crate::observer::{Observer, RefObserver, MutRefObserver};
use crate::observable::observable_source::ObservableSource;
use crate::observable::observable_source::ResultSource;

pub struct ObservableResult<T, P> {
    consumed: Vec<Result<T, P>>,
}

impl<T, P> ObservableResult<T, P> {

    pub fn new() -> ObservableResult<T, P> {
        ObservableResult {
            consumed: Vec::new(),
        }
    }
}

impl<T, P> ObservableEmitter<Result<T, P>, T, P> for ObservableResult<T, P> {
    fn next(&mut self, next: Result<T, P>) {
        self.consumed.push(next);
    }
}

impl<T, P, F, E> ObservableSource<T, P, F, E> for ObservableResult<T, P>
    where F: FnMut(T),
          E: FnMut(P) {
    fn subscribe<O>(self, mut subscriber: O) where O: MutRefObserver<T, P, F, E> {
        for result in self.consumed.into_iter() {
            result.subscribe(&mut subscriber)
        }
    }
}