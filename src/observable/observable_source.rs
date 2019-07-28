use crate::observer::{Observer, MutRefObserver};
use crate::reactive::ResultObserver;

pub trait ObservableSource<T, P, F, E>: Sized
    where F: FnMut(T),
          E: FnMut(P) {
    fn subscribe<O>(self, subscriber: O) where O: MutRefObserver<T, P, F, E>;
}

pub trait ResultSource<T, P, F, E>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn subscribe<O>(self, subscriber: O) where O: Observer<T, P, F, E>;
}

impl<T, P, F, E> ResultSource<T, P, F, E> for Result<T, P>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn subscribe<O>(self, subscriber: O) where O: Observer<T, P, F, E> {
        match self {
            Ok(next) => subscriber.next(next),
            Err(error) => subscriber.error(error),
        }
    }
}