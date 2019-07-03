use crate::observer::Observer;
use super::CurrentObserver;
use super::Single;

//TODO observable from iterator struct
impl<'a, S> From<S> for Single<'a, S>
    where S: Iterator<Item=S> {
    fn from(source: S) -> Self {
        Single::new(source)
    }
}

pub trait ObservableSource<'a, O, S>
    where O: Observer<S> + Sized,
          S: Sized {
    fn subscribe(&mut self, subscriber: &'a O);
}