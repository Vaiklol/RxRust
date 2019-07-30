use crate::disposable::Disposable;

pub trait Observer<T, P>{
    fn next(self, _: T);
    fn error(self, _: P);
}

pub trait MutRefObserver<T, P> {
    fn next_mut(&mut self, _: T);
    fn error_mut(&mut self, _: P);
}

pub trait RefObserver<T, P> {
    fn next_ref(&self, _: T);
    fn error_ref(&self, _: P);
}


mod impls {
    use super::*;

    impl<T, P, O> RefObserver<T, P> for &O
        where O: RefObserver<T, P> {
        fn next_ref(&self, next: T) {
            (**self).next_ref(next)
        }

        fn error_ref(&self, error: P) {
            (**self).error_ref(error)
        }
    }

    impl<T, P, O> MutRefObserver<T, P> for &O
        where O: RefObserver<T, P> {
        fn next_mut(&mut self, next: T) {
            (**self).next_ref(next)
        }

        fn error_mut(&mut self, error: P) {
            (**self).error_ref(error)
        }
    }

    impl<T, P, O> Observer<T, P> for &O
        where O: RefObserver<T, P> {
        fn next(self, next: T) {
            (*self).next_ref(next)
        }

        fn error(self, error: P) {
            (*self).error_ref(error)
        }
    }

    impl<T, P, O> MutRefObserver<T, P> for &mut O
        where O: MutRefObserver<T, P> {
        fn next_mut(&mut self, next: T) {
            (*self).next_mut(next)
        }

        fn error_mut(&mut self, error: P) {
            (*self).error_mut(error)
        }
    }

    impl<T, P, O> Observer<T, P> for &mut O
        where O: MutRefObserver<T, P> {
        fn next(self, next: T) {
            (*self).next_mut(next)
        }

        fn error(self, error: P) {
            (*self).error_mut(error)
        }
    }
}