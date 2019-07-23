use crate::observable::observable_emitter::ObservableEmitter;
use crate::observer::ResultObserver;
use crate::observable::observable_source::ObservableSource;
use crate::reactive::Observer;

pub struct ObservableResult<T, P> {
    consumed: Vec<Result<T, P>>,
}

impl<T, P> ObservableEmitter<Result<T, P>, T, P> for ObservableResult<T, P> {
    fn next(&mut self, next: Result<T, P>) {
        self.consumed.push(next)
    }
}

//impl<T, P, F, E> ObservableSource<T, P, F, E> for ObservableResult<T, P>
//    where F: FnOnce(T),
//          E: FnOnce(P) {
//    fn subscribe(self, subscriber: impl Observer<T, P, F, E>) {
//        self.consumed.into_iter().for_each(|next| {
//            next.subscribe(&subscriber)
//        })
//    }
//}