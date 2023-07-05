#[derive(Debug)]
pub enum Error {
    Unknown,
    Message(String),
    FromUtf8(std::string::FromUtf8Error),
    FromFloat(std::num::ParseFloatError),
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err : std::string::FromUtf8Error) -> Error {
        Error::FromUtf8(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err : std::num::ParseFloatError) -> Error {
        Error::FromFloat(err)
    }
}
