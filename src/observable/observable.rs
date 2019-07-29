use crate::observable::observable_emitter::ObservableEmitter;
use crate::observer::{MutRefObserver};
use crate::observable::observable_source::{SubscriberSource, OptionSource, ObservableSource};

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

pub struct ObservableOption<T> {
    consumed: Vec<Option<T>>,
}

impl<T> ObservableOption<T> {
    pub fn new() -> ObservableOption<T> {
        ObservableOption {
            consumed: Vec::new(),
        }
    }
}

impl<T, P> ObservableEmitter<Option<T>, T, P> for ObservableOption<T> {
    fn next(&mut self, next: Option<T>) {
        self.consumed.push(next);
    }
}

impl<T, P, F, E> ObservableSource<T, Option<P>, F, E> for ObservableOption<T>
    where F: FnMut(T),
          E: FnMut(Option<P>) {
    fn subscribe<O>(self, mut subscriber: O) where O: MutRefObserver<T, Option<P>, F, E> {
        for result in self.consumed.into_iter() {
            result.subscribe(&mut subscriber)
        }
    }
}