use crate::observable::observable_emitter::ObservableEmitter;
use crate::observer::MutRefObserver;
use crate::observable::observable_source::{OnceSource, MutSource, RefSource};
use crate::reactive::Observer;

pub struct ObservableResult<T, P, O>
    where O: MutRefObserver<T, P> {
    consumed: Vec<Result<T, P>>,
    observer: Option<O>,
}

impl<T, P, O> ObservableResult<T, P, O>
    where O: MutRefObserver<T, P> {
    pub fn new() -> ObservableResult<T, P, O> {
        ObservableResult {
            consumed: Vec::new(),
            observer: None,
        }
    }
}

impl<T, P, O> ObservableEmitter<Result<T, P>, T, P> for ObservableResult<T, P, O>
    where O: MutRefObserver<T, P> {
    fn next(&mut self, next: Result<T, P>) {
        match self.observer {
            None => self.consumed.push(next),
            Some(ref mut observer) => next.subscribe(observer)
        }
    }
}

impl<T, P, O> MutSource<T, P, O> for ObservableResult<T, P, O>
    where O: MutRefObserver<T, P> {
    fn subscribe(&mut self, mut subscriber: O) {
        self.observer = Some(subscriber);
        for result in self.consumed.drain(..) {
            result.subscribe(self.observer.as_mut().unwrap())
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

impl<T, P, O> MutSource<T, Option<P>, O> for ObservableOption<T> where O: MutRefObserver<T, Option<P>> {
    fn subscribe(&mut self, mut subscriber: O) {
        for result in self.consumed.drain(..) {
            result.subscribe(&mut subscriber)
        }
    }
}