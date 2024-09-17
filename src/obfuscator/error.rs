#[derive(Debug)]
pub enum ObfuscatorError {
    InvalidCode,
    DeadCode,
    Booleans,
    Strings,
    Numbers,
    Functions(String),
}

pub type Result<T> = std::result::Result<T, ObfuscatorError>;

impl std::fmt::Display for ObfuscatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCode => write!(
                f,
                "The code was unable to be parsed, most likely invalid python code."
            ),
            Self::Functions(s) => write!(
                f,
                "Obfuscator failed while obfuscation functions at function {s}"
            ),
            _ => write!(
                f,
                "Obfuscator encountered an error in {:?} obfuctation",
                self
            ),
        }
    }
}

impl std::error::Error for ObfuscatorError {}