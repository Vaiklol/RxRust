#[cfg(test)]
mod observable_test {
    extern crate rx;

    use rx::reactive::{
        Single,
        ObservableSource,
        Observer,
    };
    use std::cell::{RefCell, Ref};
    use std::rc::Rc;
    use std::ptr::{null, null_mut};
    use self::rx::reactive::{ObservableFrom};

    struct TestObserver(Rc<RefCell<TestObject>>);

    struct TestObject(bool);

    impl TestObject {
        fn called(&mut self, is: bool) {
            self.0 = is;
        }
    }

    impl Observer<String> for TestObserver {
        fn on_next(&self, next: String) {
            assert_eq!(next, "S".to_string());
        }

        fn on_complete(&self) {
            self.0.borrow_mut().0 = true;
        }
    }

    #[test]
    fn should_call_next_on_subscribe() {
        let mut test_object = Rc::new(RefCell::new(TestObject(false)));
        let observer = TestObserver(test_object.clone());
        Single::new("S".into())
            .subscribe(&observer);
    }

    #[test]
    fn should_call_next_on_subscribe_late() {
        let mut test_object = Rc::new(RefCell::new(TestObject(false)));
        let observer = TestObserver(test_object.clone());
        let mut observable = Single::new("S".to_owned());
        observable.subscribe(&observer);
    }

    #[test]
    fn should_call_on_complete() {
        let mut test_object = Rc::new(RefCell::new(TestObject(false)));
        {
            let observer = TestObserver(test_object.clone());
            let mut observable = Single::new("S".to_owned());
            observable.subscribe(&observer);
        }
        let is_called = test_object.borrow().0;
        assert_eq!(is_called, true)
    }

    #[test]
    fn should_use_side_operator_on_observable() {
        let iterator = vec![1, 1, 1, 1];
        let mut test_object = Rc::new(RefCell::new(TestObject(false)));
        let observer = TestObserver(test_object.clone());
        iterator.iter().map().subscribe()
    }
}