use crate::observer::Observer;
use crate::errors::{Err, SingleEmptyError};
use super::CurrentObserver;
use super::ObservableEmitter;
use super::ObservableSource;

pub struct Single<'a, S> {
    observer: Option<CurrentObserver<'a, S>>,
    source: Option<S>,
}


impl<'a, S> Single<'a, S> {
    pub fn new(source: S) -> Single<'a, S> {
        Single {
            observer: None,
            source: Some(source),
        }
    }

    fn emit(&mut self) {
        if let Some(ref observer) = self.observer {
            let mut next = None;
            std::mem::swap(&mut self.source, &mut next);
            match next {
                Some(s) => {
                    observer.on_next(s);
                    observer.on_complete()
                }
                None => {
                    let err = SingleEmptyError::new();
                    observer.on_error(Err::from(err))
                }
            }
        }
    }
}

impl<'a, O, S> ObservableSource<'a, O, S> for Single<'a, S>
    where O: Observer<S> + Sized,
          S: Sized {
    fn subscribe(&mut self, subscriber: &'a O) {
        self.observer = Some(CurrentObserver::new(subscriber));
        self.emit()
    }
}

impl<'a, S> ObservableEmitter<S> for Single<'a, S>
    where S: Sized {
    fn next(&self, next: S) {
        unimplemented!()
    }
}