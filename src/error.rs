use std;


#[derive(Debug)]
pub enum Error {
    BadSample,
    Diverged,
    EmptySample,
    Undefined,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadSample => "All sample data must be finite",
            Error::Diverged => "Numeric evaluation diverged",
            Error::EmptySample => "Sample data set cannot be empty",
            Error::Undefined => "Function undefined for argument",
        }
    }
}
