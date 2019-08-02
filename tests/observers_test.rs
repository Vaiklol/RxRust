mod observers_test {
    use rx::reactive::{Observer, Subscriber, RefObserver, MutSource, Observables, ObservableEmitter, MutRefObserver};
    use rx::reactive::OnceSource;

    #[test]
    fn should_use_observer_from_result_ok_with_once() {
        let ok: Result<u32, String> = Ok(1);
        let mut is_called = false;
        let sub = Subscriber {
            next: |_n| is_called = true,
            error: |_e| (),
        };
        ok.subscribe(sub);
        assert!(is_called)
    }

    #[test]
    fn should_use_observer_from_result_ok() {
        let ok: Result<u32, String> = Ok(1);
        let mut is_called = false;
        ok.subscribe(Subscriber::new(|_n| is_called = true, |_e| ()));
        assert!(is_called);
    }

    #[test]
    fn should_use_observer_from_result_error() {
        let ok: Result<u32, String> = Err("error".to_owned());
        let mut is_called = false;
        ok.subscribe(&mut Subscriber::new(|_n| (), |_e| is_called = true));
        assert!(is_called);
    }

    #[test]
    fn should_use_observer_from_ref_result_error() {
        let ok: Result<u32, String> = Err(String::new());
        let mut is_called = false;
        ok.as_ref()
            .subscribe(&mut Subscriber::new(|_n| (), |_e| is_called = !is_called));
        assert!(is_called);
    }

    #[test]
    fn should_use_observable_from_result() {
        let ok: Result<u32, String> = Err("error".to_owned());
        let mut is_called_err = false;
        let mut is_called_next = false;
        let mut observable = Observables::result_observable(|sub| {
            sub.next("");
            sub.error("");
        });
        observable.subscribe(Subscriber::new(|_| is_called_next = true, |_| is_called_err = true));
        assert!(is_called_err);
        assert!(is_called_next);
    }
//
//    #[test]
//    fn should_use_observable_from_mut_result() {
//        let ok: Result<u32, String> = Err("error".to_owned());
//        let mut is_called_err = false;
//        let mut is_called_next = false;
//        let mut observable = ObservableResult::new();
//        observable.next(ok);
//        observable.next(Ok(32));
//        observable.subscribe(&mut Subscriber {
//            next: |n| is_called_next = true,
//            error: |_| is_called_err = true,
//        });
//        assert!(is_called_err);
//        assert!(is_called_next);
//    }
//
//
//    #[test]
//    fn should_use_custom_observer() {
//
//        struct F(i32, i32);
//
//        impl MutRefObserver<i32, i32> for F {
//
//            fn next_mut(&mut self, next: i32) {
//                self.0 = next
//            }
//
//            fn error_mut(&mut self, error: i32) {
//               self.1 = error
//            }
//        }
//
//        let mut observer = F(0, 0);
//        let ok: Result<i32, i32> = Err(1);
//        let is_called_err = 1;
//        let is_called_next = 2;
//        let mut observable = ObservableResult::new();
//        observable.next(ok);
//        observable.subscribe(&mut observer);
//        observable.next(Ok(2));
//        assert_eq!(is_called_next, observer.0);
//        assert_eq!(is_called_err, observer.1);
//    }
}


