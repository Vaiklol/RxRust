use crate::disposable::Disposable;

pub trait Observer<T, P,> {
    fn next(self, _: T);
    fn error(self, _: P);
}

pub trait MutRefObserver<T, P, F, E>: Observer<T, P, F, E>
    where F: FnMut(T),
          E: FnMut(P) {
    fn next_mut(&mut self, _: T);
    fn error_mut(&mut self, _: P);
}

pub trait RefObserver<T, P, F, E>: MutRefObserver<T, P, F, E>
    where F: Fn(T),
          E: Fn(P) {
    fn next_ref(&self, _: T);
    fn error_ref(&self, _: P);
}

mod impls {
    use super::*;

    impl<T, P, F, E, O> RefObserver<T, P, F, E> for &O
        where F: Fn(T),
              E: Fn(P),
              O: RefObserver<T, P, F, E> {
        fn next_ref(&self, next: T) {
            (**self).next_ref(next)
        }

        fn error_ref(&self, error: P) {
            (**self).error_ref(error)
        }
    }

    impl<T, P, F, E, O> MutRefObserver<T, P, F, E> for &O
        where F: Fn(T),
              E: Fn(P),
              O: RefObserver<T, P, F, E> {
        fn next_mut(&mut self, next: T) {
            (**self).next_ref(next)
        }

        fn error_mut(&mut self, error: P) {
            (**self).error_ref(error)
        }
    }

    impl<T, P, F, E, O> Observer<T, P, F, E> for &O
        where F: Fn(T),
              E: Fn(P),
              O: RefObserver<T, P, F, E> {
        fn next(self, next: T) {
            (*self).next_ref(next)
        }

        fn error(self, error: P) {
            (*self).error_ref(error)
        }
    }

    impl<T, P, F, E, O> MutRefObserver<T, P, F, E> for &mut O
        where F: FnMut(T),
              E: FnMut(P),
              O: MutRefObserver<T, P, F, E> {
        fn next_mut(&mut self, next: T) {
            (*self).next_mut(next)
        }

        fn error_mut(&mut self, error: P) {
            (*self).error_mut(error)
        }
    }

    impl<T, P, F, E, O> Observer<T, P, F, E> for &mut O
        where F: FnMut(T),
              E: FnMut(P),
              O: MutRefObserver<T, P, F, E> {
        fn next(self, next: T) {
            (*self).next_mut(next)
        }

        fn error(self, error: P) {
            (*self).error_mut(error)
        }
    }
}