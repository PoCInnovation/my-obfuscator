use super::Result;
use super::{error::ObfuscatorError, Obfuscator};
use tree_sitter::Parser;

pub const OBFUSCATOR_HELPER_FUNCTIONS: &str = r#"
def ohe_string_decode(string):
    string = list(string)
    if string == []:
        return str()
    for i in range(len(string)):
        if ord(string[i]) >= 35 and ord(string[i]) <= 125 and 0:
            string[i] = chr(ord(string[i]) - 1)
    return ''.join(string)

import sys
if (gettrace := getattr(sys, 'gettrace')) and gettrace() or 'pdb' in sys.modules or 'debugpy' in sys.modules or 'pydevd' in sys.modules or 'ptvsd' in sys.modules or 'wdb' in sys.modules:
    import os;os._exit(0)

def ohe_useless(*args, **kwargs):
    return

def ohe_thruthy(*args, **kwargs):
    return ohe_useless(args, kwargs) or 1 == int(float("01.0342671"))

def ohe_falsy(*args, **kwargs):
    return ohe_thruthy(args, value="awae", iteration=2) and str(2) == "the_number_two"

def ohe_call_function(function_call_string):
    import re, ast, builtins
    match = re.match(r'(\w+)\((.*)\)$', function_call_string)
    ohe = lambda: eval(function_call_string, globals(), locals())
    if not match:
        return ohe()

    func_name, args_string = match.groups()
    args = []
    kwargs = {}
    if args_string:
        try:
            parsed = ast.parse(f"swagy_func({args_string})").body[0].value.args
            args = [ast.literal_eval(arg) for arg in parsed if isinstance(arg, ast.Constant)]
            kwargs = {kw.arg: ast.literal_eval(kw.value) for kw in parsed if isinstance(kw, ast.keyword)}
        except (SyntaxError, ValueError):
            raise ValueError("Invalid arguments in function call string")

    if func_name in dir(builtins):
        ohe_fun = getattr(builtins, func_name)
    else:
        ohe_fun = globals().get(func_name)


    if ohe_fun is None:
        return ohe()

    return ohe_fun(*args, **kwargs)


"#;

impl Obfuscator {
    pub fn new(mut code: String) -> Result<Self> {
        let mut parser = Parser::new();

        code.insert_str(0, OBFUSCATOR_HELPER_FUNCTIONS);
        parser
            .set_language(&tree_sitter_python::language())
            .expect("error setting language");
        let tree = parser.parse(&code, None);
        if let Some(tree) = tree {
            Ok(Obfuscator { code, parser, tree })
        } else {
            Err(ObfuscatorError::InvalidCode)
        }
    }

    pub fn reparse(&mut self, error_case: ObfuscatorError) -> Result<()> {
        if let Some(new_tree) = self.parser.parse(&self.code, None) {
            self.tree = new_tree;
            Ok(())
        } else {
            Err(error_case)
        }
    }
}
