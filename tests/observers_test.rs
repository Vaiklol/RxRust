mod observers_test {
    use rx::reactive::{Observer, ResultObserver, ObservableSource};

    #[test]
    fn should_observer_from_result_ok() {
        let ok: Result<u32, String> = Ok(1);
        let mut is_called = false;
        ok.subscribe(ResultObserver {
            next: |n| is_called = true,
            error: |_| ()
        });
        assert!(is_called);
    }

    #[test]
    fn should_observer_from_result_error() {
        let ok: Result<u32, String> = Err("error".to_owned());
        let mut is_called = false;
        ok.subscribe(ResultObserver {
            next: |n| (),
            error: |_| is_called = true
        });
        assert!(is_called);
    }
}