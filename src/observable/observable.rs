//use crate::observable::ObservableEmitter;
//use crate::observable::ObservableSource;
//use crate::observer::ResultObserver;
//use crate::observer::Observer;
//use crate::observer::OptionObserver;
//use std::mem::swap;
//
//pub struct ResultObservable<'a, T, E> {
//    observer: Option<ResultObserver<'a, T, E>>,
//    source: Option<Result<T, E>>,
//}
//
//macro_rules! impl_source_result  {
//    ($t:ty, $o:ty) => {
//        impl<'a, T: 'a, E: 'a> ObservableSource<'a> for $t {
//            type Observer = $o;
//
//            fn subscribe(self, subscriber: Self::Observer) {
//                match self {
//                    Ok(ok) => subscriber.on_next(ok),
//                    Err(err) => subscriber.on_error(err),
//                }
//            }
//        }
//    }
//}
//
//impl_source_result!(Result<T, E>, ResultObserver<'a, T, E>);
//impl_source_result!(&'a Result<T, E>, ResultObserver<'a, &'a T, &'a E>);
//impl_source_result!(&'a mut Result<T, E>, ResultObserver<'a, &'a mut T, &'a mut E>);
//
//macro_rules! impl_source_result  {
//    ($t:ty, $o:ty) => {
//        impl<'a, T: 'a> ObservableSource<'a> for $t {
//            type Observer = $o;
//
//            fn subscribe(self, subscriber: Self::Observer) {
//                match self {
//                    Some(ok) => subscriber.on_next(ok),
//                    None => subscriber.on_error(()),
//                }
//            }
//        }
//    }
//}
//
//impl_source_result!(Option<T>, OptionObserver<'a, T>);
//impl_source_result!(&'a Option<T>, OptionObserver<'a, &'a T>);
//impl_source_result!(&'a mut Option<T>, OptionObserver<'a, &'a mut T>);