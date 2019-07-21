pub trait ObservableSource<'a>: Sized {
    type Observer;

    fn subscribe(self, subscriber: Self::Observer) {}
}