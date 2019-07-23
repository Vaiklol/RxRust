use crate::observer::Observer;
use crate::reactive::ResultObserver;

pub trait ObservableSource<T, P, F, E>: Sized
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn subscribe(self, subscriber: impl Observer<T, P, F, E>);
}

impl<T, P, F, E> ObservableSource<T, P, F, E> for Result<T, P>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn subscribe(self, subscriber: impl Observer<T, P, F, E>) {
        match self {
            Ok(next) => subscriber.next(next),
            Err(error) => subscriber.error(error),
        }
    }
}