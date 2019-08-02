use crate::observable::observable_emitter::ObservableEmitter;
use crate::observer::MutRefObserver;
use crate::observable::observable_source::{OnceSource, MutSource, RefSource};
use crate::reactive::Observer;

struct Observable<F, O> where F: FnOnce(O){
    current_observer: Option<O>,
    observable_emitter: F,
}

pub trait Observables {
    fn result_observable<T, P, O, F>(observable_emitter: F) -> Observable<F, O>
        where O: Observer<T, P>,
              F: FnOnce(O) {
        Observable {
            current_observer: None,
            observable_emitter,
        }
    }
}

//impl<T, P, O> ObservableResult<T, P, O>
//    where O: MutRefObserver<T, P> {
//    pub fn new() -> ObservableResult<T, P, O> {
//        ObservableResult {
//            consumed: Vec::new(),
//            observers: Vec::new(),
//        }
//    }
//}

//impl<T, P, O> ObservableEmitter<Result<T, P>, T, P> for ObservableResult<T, P, O>
//    where O: MutRefObserver<T, P> {
//    fn next(&mut self, next: Result<T, P>) {
//        self.observers.iter_mut().for_each(|o| {
//            match o {
//                None => self.consumed.push(next),
//                Some(ref mut observer) => next.subscribe(observer)
//            }
//        })
//    }
//}

impl<T, P, O, F> MutSource<T, P, O> for Observable<F, O>
    where O: MutRefObserver<T, P>,
          F: FnMut(O) {
    fn subscribe(&mut self, subscriber: O) {
        (self.observable_emitter)(subscriber)
    }
}

//pub struct ObservableOption<T> {
//    consumed: Vec<Option<T>>,
//}

//impl<T> ObservableOption<T> {
//    pub fn new() -> ObservableOption<T> {
//        ObservableOption {
//            consumed: Vec::new(),
//        }
//    }
//}

//impl<T, P> ObservableEmitter<Option<T>, T, P> for ObservableOption<T> {
//    fn next(&mut self, next: Option<T>) {
//        self.consumed.push(next);
//    }
//
//    fn error(&mut self, error: P) {
//        unimplemented!()
//    }
//}

//impl<T, P, O> MutSource<T, Option<P>, O> for ObservableOption<T> where O: MutRefObserver<T, Option<P>> {
//    fn subscribe(&mut self, mut subscriber: O) {
//        for result in self.consumed.drain(..) {
//            result.subscribe(&mut subscriber)
//        }
//    }
//}