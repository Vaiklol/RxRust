use crate::disposable::Disposable;

#[macro_export]
macro_rules! empty_fn {
    () => {
        |_| ()
     }
}

trait Observer<T, P, F>
    where F: FnOnce(T) {
    type Next;
    fn next(self, _: Self::Next);
    fn error(self, _: P);
}

struct ResultObserver<F, E> {
    next: F,
    error: E,
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

impl<F, E> Default for ResultObserver<F, E>
    where F: FnOnce(()),
          E: FnOnce(()) {
    fn default() -> Self {
        ResultObserver {
            next: empty_fn!(),
            error: empty_fn!(),
        }
    }
}
//
//pub struct ResultObserver<'a, T: 'a, E: 'a> {
//    pub next: &'a FnOnce(T),
//    pub error: &'a FnOnce(E),
//}
//
//
//impl<'a, T, E> Observer<T> for ResultObserver<'a, T, E> {
//    type Error = E;
//
//    fn on_next(self, next: T) {
//        (self.next)(next)
//    }
//
//    fn on_error(self, error: Self::Error) {
//        (self.error)(error)
//    }
//}
//
//pub struct OptionObserver<'a, T: 'a> {
//    pub next: &'a Fn(T),
//    pub none: &'a Fn(()),
//}
//
//impl<'a, T> Default for OptionObserver<'a, T> {
//    fn default() -> Self {
//        OptionObserver {
//            next: empty_fn!(),
//            none: empty_fn!(),
//        }
//    }
//}
//
//impl<'a, T> Observer<T> for OptionObserver<'a, T> {
//    type Error = ();
//
//    fn on_next(self, next: T) {
//        (self.next)(next)
//    }
//
//    fn on_error(self, error: Self::Error) {
//        (self.none)(error)
//    }
//}