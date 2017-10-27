use std;


#[derive(Debug)]
pub enum Error {
    BadSample,
    EmptySample,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadSample => "All sample data must be finite",
            Error::EmptySample => "Sample data set cannot be empty",
        }
    }
}
