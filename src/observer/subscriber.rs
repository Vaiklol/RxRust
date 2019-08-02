use crate::observer::{Observer, RefObserver, MutRefObserver};

pub struct Subscriber<T, P> {
    pub next: T,
    pub error: P,
}

impl<T, P> Subscriber<T, P> {
    pub fn new(next: T, error: P) -> impl Observer<T, P> {
        Subscriber {
            next,
            error,
        }
    }
}

impl<T, P, F, E> Observer<T, P, > for Subscriber<F, E>
    where F: FnOnce(T),
          E: FnOnce(P) {
    fn next(self, next: T) {
        (self.next)(next)
    }

    fn error(self, error: P) {
        (self.error)(error)
    }
}

impl<T, P, F, E> MutRefObserver<T, P, > for Subscriber<F, E>
    where F: FnMut(T),
          E: FnMut(P) {
    fn next_mut(&mut self, next: T) {
        (self.next)(next)
    }

    fn error_mut(&mut self, error: P) {
        (self.error)(error)
    }
}

impl<T, P, F, E> RefObserver<T, P, > for Subscriber<F, E>
    where F: Fn(T),
          E: Fn(P) {
    fn next_ref(&self, next: T) {
        (self.next)(next)
    }

    fn error_ref(&self, error: P) {
        (self.error)(error)
    }
}