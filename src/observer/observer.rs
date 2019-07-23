use crate::disposable::Disposable;

pub trait Observer<T, P, F, E>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn next(self, _: T);
    fn error(self, _: P);
}

pub trait RefObserver<T, P, F, E>: MutRefObserver<T, P, F, E>
    where F: Fn(T),
          E: Fn(P) {
    fn next(&self, _: T);
    fn error(&self, _: P);
}

pub trait MutRefObserver<T, P, F, E>: Observer<T, P, F, E>
    where F: FnMut(T),
          E: FnMut(P) {
    fn next(&mut self, _: T);
    fn error(&mut self, _: P);
}

pub struct ResultObserver<F, E> {
    pub next: F,
    pub error: E,
}

impl<'a, T, P, F, E, R> Observer<T, P, F, E> for &'a R
    where F: Fn(T),
          E: Fn(P),
          &'a R: RefObserver<T, P, F, E> + Sized {
    fn next(self, next: T) {
        RefObserver::next(&self, next)
    }

    fn error(self, error: P) {
        RefObserver::error(&self, error)
    }
}

impl<'a, T, P, F, E, R> Observer<T, P, F, E> for &'a mut R
    where F: FnMut(T),
          E: FnMut(P),
          &'a mut R: MutRefObserver<T, P, F, E> + Sized {
    fn next(mut self, next: T) {
        MutRefObserver::next(&mut self, next)
    }

    fn error(mut self, error: P) {
        MutRefObserver::error(&mut self, error)
    }
}

impl<T, P, F, E> Observer<T, P, F, E> for ResultObserver<F, E>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn next(self, next: T) {
        (self.next)(next)
    }

    fn error(self, error: P) {
        (self.error)(error)
    }
}

//impl<T, P, F, E> RefObserver<T, P, F, E> for &ResultObserver<F, E>
//    where F: Fn(T),
//          E: Fn(P) {
//    fn next(&self, next: T) {
//        (self.next)(next)
//    }
//
//    fn error(&self, error: P) {
//        (self.error)(error)
//    }
//}
//
//impl<T, P, F, E> MutRefObserver<T, P, F, E> for &mut ResultObserver<F, E>
//    where F: FnMut(T),
//          E: FnMut(P) {
//    fn next(&mut self, next: T) {
//        (self.next)(next)
//    }
//
//    fn error(&mut self, error: P) {
//        (self.error)(error)
//    }
//}