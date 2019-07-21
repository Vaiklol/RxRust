use crate::disposable::Disposable;

pub trait Observer<T, P, F>
    where F: FnOnce(T) {
    type Next;
    fn next(self, _: Self::Next);
    fn error(self, _: P);
}

pub struct ResultObserver<F, E> {
    pub next: F,
    pub error: E,
}

impl<T, P, F, E> Observer<T, P, F> for ResultObserver<F, E>
    where F: FnOnce(T),
          E: FnOnce(P) {
    type Next = T;

    fn next(self, next: Self::Next) {
        (self.next)(next)
    }

    fn error(self, error: P) {
        (self.error)(error)
    }
}