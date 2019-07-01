use std::error::Error;
use crate::disposable::Disposable;

trait Observer<T> {

    fn on_next(next: T) {}
    fn on_complete() {}
    fn on_error(err: impl Error) {}
    fn on_subscribe(disposable: Disposable) {}
}