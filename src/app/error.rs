#[derive(Debug, Clone)]
pub enum Error {
    APIError,
    LanguageError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        dbg!(error);
        Self::APIError
    }
}
