use super::error::ObfuscatorError;
use super::Obfuscator;
use super::Result;

impl Obfuscator {
    pub fn remove_comments_and_empty_lines(&mut self) -> Result<()> {
        self.code = self
            .code
            .lines()
            .filter(|line| {!line.trim().is_empty()})
            .collect::<Vec<_>>()
            .join("\n");
        self.reparse(ObfuscatorError::RemoveCommentsAndEmptyLines)
    }
}
