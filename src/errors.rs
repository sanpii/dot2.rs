pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidId,
}

impl std::error::Error for Error {
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::InvalidId => "Invalid id".to_string(),
            Self::Io(err) => format!("{}", err),
        };

        write!(f, "{s}")
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
