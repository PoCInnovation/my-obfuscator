use super::error::ObfuscatorError;
use super::Obfuscator;
use super::Result;
use super::Shiftable;
use tree_sitter::{Tree, TreeCursor};

fn get_comments(tree: &Tree) -> Vec<std::ops::Range<usize>> {
    fn go(cursor: &mut TreeCursor, comments: &mut Vec<std::ops::Range<usize>>) {
        let node = cursor.node();
        if node.kind() == "comment" {
            comments.push(node.start_byte()..node.end_byte());
        }
        if cursor.goto_first_child() {
            go(cursor, comments);
            cursor.goto_parent();
        }
        while cursor.goto_next_sibling() {
            go(cursor, comments);
        }
    }
    let mut comments = Vec::new();

    go(&mut tree.walk(), &mut comments);

    comments
}

impl Obfuscator {
    pub fn remove_comments_and_empty_lines(&mut self) -> Result<()> {
        let comments = get_comments(&self.tree);

        let mut shift = 0;
        comments.into_iter().for_each(|comment| {
            let len = comment.len();

            self.code.replace_range(comment.shift(shift), "");
            shift -= len as i32;
        });
        self.code = self
            .code
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        self.reparse(ObfuscatorError::RemoveCommentsAndEmptyLines)
    }
}
