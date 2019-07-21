// Copyright (C) 2015 <Rick Richardson r@12sidedtech.com>
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.
//! # Reactive Rust
//! A reactive streams library in the vein of [reactive-streams.org](http://reactive-streams.org)
//! with inspiration taken from [Elm](http://elm-lang.org)
//! Reactive streams are a mechanism for chaining computation in an event-driven way.
//! If it helps, think of them as Iterators but Push instead of Pull
//!
//! This library is still early alpha, and surely has bugs.
//!
//! ## Goals
//!
//! * Speed - Rust is a systems library. Convieniences
//!    are foresaken for performance where necessary
//! * 0-Copy - The core IO mechanisms pass around refcounted handles to buffers,
//!    data passing is typically move where possible, clone/copy is rarely required
//! * Extensibility - It is assumed that the user of the API will want their own
//!    objects to be easily made reactive.
//! * Familiarity - The "builder" API has a similar look and feel as the
//!     stdlibs Iterator API, but reactive streams themselves are much more
//!     loosely coupled
//!
//! The core abstraction of Reactive Streams is a [Publisher](reactive/trait.Publisher.html) /
//! [Subscriber](reactive/trait.Subscriber)
//!
//! Objects which are both a Publisher and a Subscriber are a [Processor](reactive/trait.Processor.html)
//!
//!
//! ## Example
//!
//! ```rust
//! ```
#![doc(html_root_url = "http://www.rust-ci.org/rrichardson/reactive/doc/reactive/")]
#![crate_id = "rx"]
#![crate_type = "lib"]

#[macro_use]
mod default_macros;
mod observable;
#[macro_use]
mod observer;
mod disposable;

pub mod reactive {
//    pub use crate::observable::{ObservableSource, ResultObservable, ObservableEmitter};
//    pub use crate::observer::Observer;
//    pub use crate::observer::{ResultObserver, OptionObserver};
}
