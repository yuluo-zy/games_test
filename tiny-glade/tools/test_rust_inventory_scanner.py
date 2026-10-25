"""Boundary checks for the lexical inventory scan without running the audit."""
import ast
from pathlib import Path
import re
import unittest

module = ast.parse(Path(__file__).with_name("verify_inventory.py").read_text(encoding="utf-8"))
definitions = [node for node in module.body if isinstance(node, ast.FunctionDef) and node.name == "rust_code"
    or isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id.startswith("RUST_") for target in node.targets)]
namespace = {"re": re}
exec(compile(ast.Module(body=definitions, type_ignores=[]), "inventory-scanner", "exec"), namespace)
rust_code = namespace["rust_code"]
stub = re.compile(r"\b(?:todo|unimplemented)\s*!\s*\(")

class RustInventoryLexing(unittest.TestCase):
    def test_nested_comments_are_not_executable_stubs(self):
        self.assertFalse(stub.search(rust_code('/* todo!() /* unimplemented!() */ */ fn f() {}', True)))

    def test_raw_and_escaped_strings_cannot_create_comment_or_stub_tokens(self):
        source = 'let a = r##"todo!() // /*"##; let b = "quoted \\\" // unimplemented!()";'
        self.assertFalse(stub.search(rust_code(source, True)))
        self.assertIn('todo!() // /*', rust_code(source))

    def test_byte_raw_string_is_consumed_before_comment_markers(self):
        self.assertFalse(stub.search(rust_code('let b = br#"unimplemented!() /* //"#;', True)))

    def test_lifetimes_do_not_hide_actual_stubs(self):
        self.assertTrue(stub.search(rust_code("fn f<'a>(_: &'a str) { todo!() }", True)))

    def test_character_escapes_and_unicode_are_consumed(self):
        self.assertTrue(stub.search(rust_code("let a = '\\u{2f}'; let b = '/'; let c = '\\\''; todo!();", True)))

    def test_comments_between_macro_tokens_preserve_detection(self):
        self.assertTrue(stub.search(rust_code('todo /* historical name */ ! ("missing")', True)))

    def test_strings_are_preserved_for_evidence_path_audit(self):
        source = 'let p = "../../evidence/file.json"; // "../../evidence/old.json"'
        result = rust_code(source)
        self.assertIn('"../../evidence/file.json"', result)
        self.assertNotIn('old.json', result)

if __name__ == "__main__":
    unittest.main()
