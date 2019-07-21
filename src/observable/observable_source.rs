use crate::observer::Observer;
use crate::reactive::ResultObserver;

pub trait ObservableSource<T, P>: Sized {

    fn subscribe<F>(self, subscriber: impl Observer<T, P, F, Next=T>) where F: FnOnce(T);
}

impl<T, P> ObservableSource<T, P> for Result<T, P> {

    fn subscribe<F>(self, subscriber: impl Observer<T, P, F, Next=T>) where F: FnOnce(T) {
        match self {
            Ok(n) => subscriber.next(n),
            Err(e) => subscriber.error(e)
        }
    }
}