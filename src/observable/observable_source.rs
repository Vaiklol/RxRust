use crate::observer::{Observer, MutRefObserver, RefObserver};
use crate::reactive::Subscriber;

pub trait MutSource<T, P, O> {
    fn subscribe(&mut self, subscriber: O);
}

pub trait RefSource<T, P, O> {
    fn subscribe(&self, subscriber: O);
}

pub trait OnceSource<T, P, O> where O: Observer<T, P>{
    fn subscribe(self, subscriber: O);
}

mod impls {
    use super::*;
//
//    impl<T, P, O> RefSource<T, P, O> for &dyn RefSource<T, P, O>
//        where Self: Sized,
//              O: RefObserver<T, P> {
//        fn subscribe(&self, subscriber: O) {
//            (**self).subscribe(subscriber)
//        }
//    }
//
//    impl<T, P, O> MutSource<T, P, O> for &dyn RefSource<T, P, O>
//        where Self: Sized,
//              O: RefObserver<T, P> {
//        fn subscribe(&mut self, subscriber: O) {
//            (**self).subscribe(subscriber)
//        }
//    }
//
//    impl<T, P, O> OnceSource<T, P, O> for &dyn RefSource<T, P, O>
//        where Self: Sized,
//              O: RefObserver<T, P> {
//        fn subscribe(self, subscriber: O) {
//            (*self).subscribe(subscriber)
//        }
//    }
//
//    impl<T, P, O> MutSource<T, P, O> for &mut dyn MutSource<T, P, O>
//        where Self: Sized,
//              O: MutRefObserver<T, P> {
//        fn subscribe(&mut self, subscriber: O) {
//            (*self).subscribe(subscriber)
//        }
//    }
//
//    impl<T, P, O> OnceSource<T, P, O> for &mut dyn MutSource<T, P, O>
//        where Self: Sized,
//              O: MutRefObserver<T, P> {
//        fn subscribe(self, subscriber: O) {
//            (*self).subscribe(subscriber)
//        }
//    }
}

impl<T, P, O> OnceSource<T, P, O> for Result<T, P>
    where O: Observer<T, P> {
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