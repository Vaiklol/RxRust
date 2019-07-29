use crate::observer::{Observer, MutRefObserver};
use crate::reactive::Subscriber;

pub trait ObservableSource<T, P, F, E>: Sized
    where F: FnMut(T),
          E: FnMut(P) {
    fn subscribe<O>(self, subscriber: O) where O: MutRefObserver<T, P, F, E>;
}

pub trait SubscriberSource<T, P, F, E>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn subscribe<O>(self, subscriber: O) where O: Observer<T, P, F, E>;
}

pub trait OptionSource<T, P, F, E>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn subscribe<O>(self, subscriber: O) where O: Observer<T, P, F, E>;
}

impl<T, P, F, E> SubscriberSource<T, P, F, E> for Result<T, P>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn subscribe<O>(self, subscriber: O) where O: Observer<T, P, F, E> {
        match self {
            Ok(next) => subscriber.next(next),
            Err(error) => subscriber.error(error),
        }
    }
}

impl<T, P, F, E> OptionSource<T, Option<P>, F, E> for Option<T>
    where F: FnOnce(T),
          E: FnOnce(Option<P>) {
    fn subscribe<O>(self, subscriber: O) where O: Observer<T, Option<P>, F, E> {
        match self {
            Some(next) => subscriber.next(next),
            None => subscriber.error(None),
        }
    }
}