#[derive(Debug)]
pub enum ObfuscatorError {
    InvalidCode,
    DeadCode,
    Booleans,
    Strings,
    Numbers,
    PythonSyntaxCheck(std::io::Error),
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
            Self::PythonSyntaxCheck(err) => {
                write!(f, "The Python command for syntax failed : {err}")
            }
            _ => write!(
                f,
                "Obfuscator encountered an error in {:?} obfuctation",
                self
            ),
        }
    }
}

impl std::error::Error for ObfuscatorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PythonSyntaxCheck(err) => Some(err),
            _ => None
        }
    }
}
