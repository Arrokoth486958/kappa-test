use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub struct KappaError {
    info: String,
}

impl KappaError {
    pub fn new(info: impl Into<String>) -> Self {
        KappaError { info: info.into() }
    }
}

impl Error for KappaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for KappaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.info.as_str())
    }
}
