use crate::observer::Observer;
use crate::observable::ObservableSource;
use std::iter::Map;

impl<S, I> From<S> for ObservableFrom<S>
    where S: Iterator<Item=I> {
    fn from(source: S) -> Self {
        ObservableFrom { source }
    }
}

impl<'a, O, S, I> ObservableSource<'a, O, S> for ObservableFrom<S>
    where O: Observer<S> + Sized,
          S: Iterator<Item=I> {
    fn subscribe(&mut self, subscriber: &'a O) {}
}

impl<'a, O, S, I> ObservableSource<'a, O, S> for Iterator<Item=I>
    where O: Observer<S> + Sized {
    fn subscribe(&mut self, subscriber: &'a O) {}
}

impl<'a, O, S, I, F, G> ObservableSource<'a, O, S> for Map<S, F>
    where O: Observer<S> + Sized,
          S: Iterator<Item=I>,
          F: FnMut(S::Item) -> G {
    fn subscribe(&mut self, subscriber: &'a O) {}
}

pub struct ObservableFrom<S> {
    source: S,
}

impl<S, I> Iterator for ObservableFrom<S>
    where S: Iterator<Item=I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next()
    }
}