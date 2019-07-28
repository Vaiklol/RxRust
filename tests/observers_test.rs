mod observers_test {
    use rx::reactive::{Observer, ResultObserver, ObservableSource, ObservableResult, ObservableEmitter};
    use rx::reactive::ResultSource;

    #[test]
    fn should_observer_from_result_ok() {
        let ok: Result<u32, String> = Ok(1);
        let mut is_called = false;
        ok.subscribe(ResultObserver::new(|_| is_called = true, |_| ()));
        assert!(is_called);
    }

    #[test]
    fn should_observer_from_result_error() {
        let ok: Result<u32, String> = Err("error".to_owned());
        let mut is_called = false;
        ok.subscribe(&mut ResultObserver::new(|_| (), |_| is_called = true));
        assert!(is_called);
    }

    #[test]
    fn should_use_observable_from_result() {
        let ok: Result<u32, String> = Err("error".to_owned());
        let mut is_called_err = false;
        let mut is_called_next = false;
        let mut observable = ObservableResult::new();
        observable.next(ok);
        observable.next(Ok(32));
        observable.subscribe(ResultObserver::new(|_| is_called_next = true, |_| is_called_err = true));
        assert!(is_called_err);
        assert!(is_called_next);
    }

    #[test]
    fn should_use_observable_from_mut_result() {
        let ok: Result<u32, String> = Err("error".to_owned());
        let mut is_called_err = false;
        let mut is_called_next = false;
        let mut observable = ObservableResult::new();
        observable.next(ok);
        observable.next(Ok(32));
        observable.subscribe(&mut ResultObserver {
            next: |n| is_called_next = true,
            error: |_| is_called_err = true,
        });
        assert!(is_called_err);
        assert!(is_called_next);
    }
}