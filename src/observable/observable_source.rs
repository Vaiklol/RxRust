use crate::observer::{Observer, MutRefObserver, RefObserver};
use crate::reactive::Subscriber;

pub trait MutSource<T, P, O>: Sized {
    fn subscribe(&mut self, subscriber: O);
}

pub trait RefSource<T, P, O>: Sized {
    fn subscribe(&self, subscriber: O);
}

pub trait OnceSource<T, P, O> {
    fn subscribe(self, subscriber: O);
}

impl<T, P, O> OnceSource<T, P, O> for Result<T, P> where O: Observer<T, P> {
    fn subscribe(self, subscriber: O) {
        match self {
            Ok(next) => subscriber.next(next),
            Err(error) => subscriber.error(error),
        }
    }
}

impl<T, P, O> OnceSource<T, Option<P>, O> for Option<T> where O: Observer<T, Option<P>> {
    fn subscribe(self, subscriber: O) {
        match self {
            Some(next) => subscriber.next(next),
            None => subscriber.error(None),
        }
    }
}