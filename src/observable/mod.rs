mod observable_source;
mod observable;
mod single;
mod observable_emitter;

use crate::observer::Observer;

type CurrentObserver<'a, S> = Box<&'a dyn Observer<S>>;

pub use observable_source::{ObservableSource};
pub use observable_emitter::ObservableEmitter;
pub use single::Single;
pub use observable::{ObservableFrom};
