use tree_sitter::{Parser, Tree};

pub struct Obfuscator {
    pub code: String,
    pub parser: Parser,
    pub tree: Tree,
}

const STRING_OBFUSCATOR_HELPER: &str = r#"
def string_decode(string):
    string = list(string)
    if string == []:
        return str()
    for i in range(len(string)):
        if ord(string[i]) >= 35 and ord(string[i]) <= 125:
            string[i] = chr(ord(string[i]) - 1)
    return ''.join(string)
"#;

impl Obfuscator {
    pub fn new(mut code: String) -> Self {
        let mut parser = Parser::new();

        code.insert_str(0, STRING_OBFUSCATOR_HELPER);
        parser
            .set_language(&tree_sitter_python::language())
            .expect("error setting language");
        let tree = parser
            .parse(code.as_bytes(), None)
            .expect("error parsing code syntax error might be a reason");

        Obfuscator { code, parser, tree }
    }

    pub fn reparse(&mut self) {
        self.tree = self.parser.parse(&self.code, None).expect("error reparsing");
    }
}
