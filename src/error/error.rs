use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct TOR2Error {
    pub msg: String,
}

impl Display for TOR2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("{}", self.msg));
    }
}

impl std::error::Error for TOR2Error {}

impl TOR2Error {
    pub fn new_caused_by(err: Box<dyn std::error::Error>) -> Self {
        return Self {
            msg: err.to_string(),
        };
    }

    pub fn new_standalone(msg: String) -> Self {
        return Self {
            msg,
        };
    }
}