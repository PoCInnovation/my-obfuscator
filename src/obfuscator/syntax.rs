use super::Obfuscator;
use super::Result;
use std::process::Command;

impl Obfuscator {
    pub fn is_syntax_ok(&self) -> Result<bool> {
        let cmd_status = Command::new("python3")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .arg("-Bc")
            .arg(format!(
                "compile(r\'\'\'{}\'\'\',\"\", \"exec\")",
                self.code
            ))
            .status();

        let cmd_res = match cmd_status {
            Ok(val) => val,
            Err(err) => return Err(super::error::ObfuscatorError::PythonSyntaxCheck(err)),
        };

        if cmd_res.success() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
