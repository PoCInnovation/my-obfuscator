use super::Result;
use super::{error::ObfuscatorError, Obfuscator};
use tree_sitter::Parser;

pub const OBFUSCATOR_HELPER_FUNCTIONS: &str = r#"
import sys
if (gettrace := getattr(sys, 'gettrace')) and gettrace() or 'pdb' in sys.modules or 'debugpy' in sys.modules or 'pydevd' in sys.modules or 'ptvsd' in sys.modules or 'wdb' in sys.modules:
    import os;os._exit(0)


def string_decode(string):
    string = list(string)
    if string == []:
        return str()
    for i in range(len(string)):
        if ord(string[i]) >= 35 and ord(string[i]) <= 125:
            string[i] = chr(ord(string[i]) - 1)
    return ''.join(string)

def useless(*args, **kwargs):
    return

def thruthy(*args, **kwargs):
    return useless(args, kwargs) or 1 == int(float("01.0342671"))

def falsy(*args, **kwargs):
    return thruthy(args, value="awae", iteration=2) and str(2) == "the_number_two"

"#;

impl Obfuscator {
    pub fn new(mut code: String) -> Self {
        let mut parser = Parser::new();

        code.insert_str(0, OBFUSCATOR_HELPER_FUNCTIONS);
        parser
            .set_language(&tree_sitter_python::language())
            .expect("error setting language");
        let tree = parser
            .parse(&code, None)
            .expect("Parsing code failed likely due to invalid syntax");

        Obfuscator { code, parser, tree }
    }

    pub fn reparse<'a>(&mut self, error_case: ObfuscatorError) -> Result<()> {
        if let Some(new_tree) = self.parser.parse(&self.code, None) {
            self.tree = new_tree;
            Ok(())
        } else {
            Err(error_case)
        }
    }
}
