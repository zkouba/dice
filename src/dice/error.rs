use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DiceError {
    pub msg: String,
}

impl Display for DiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.msg))
    }
}

impl std::error::Error for DiceError {}

impl DiceError {
    pub fn new_caused_by(err: Box<dyn std::error::Error>) -> Self {
        Self {
            msg: err.to_string(),
        }
    }

    pub fn new_standalone(msg: String) -> Self {
        Self {
            msg,
        }
    }
}