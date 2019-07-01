trait ObservableSource<T>
    where T: Subscriber {

    fn subscribe(&self, subscriber: T);
}