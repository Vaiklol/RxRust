use std::error::Error;
use std::fmt::{Display, Formatter, Result};

pub type Err = Box<dyn Error>;

const DESCRIPTION: &str = "Cannot get value from None in Single after complete.";
const CAUSE: &str = "Single empty source.";

#[derive(Debug)]
pub struct SingleEmptyError {
    cause: String,
    description: String,
}

impl SingleEmptyError {
    pub fn new() -> impl Error {
        SingleEmptyError {
            cause: CAUSE.to_owned(),
            description: DESCRIPTION.to_owned(),
        }
    }
}

impl Display for SingleEmptyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f, "{} {}",
            self.cause,
            self.description
        )
    }
}

impl Error for SingleEmptyError {
    fn cause(&self) -> Option<&Error> {
        Some(self)
    }
}