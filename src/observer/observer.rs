use std::error::Error;
use crate::disposable::Disposable;
use crate::errors::Err;

#[warn(unused_variables)]
pub trait Observer<T> {
    fn on_next(&self, next: T) {}
    fn on_complete(&self) {}
    fn on_error(&self, err: Err) {}
    fn on_subscribe(&self, disposable: Disposable) {}
}