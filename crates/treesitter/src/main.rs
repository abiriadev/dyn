use tree_sitter::{Parser, TreeCursor};

fn main() {
	let mut parser = Parser::new();

	parser
		.set_language(&tree_sitter_dyn::language())
		.unwrap();

	let src = "let a = 1 + 2";

	let tree = parser.parse(src, None).unwrap();

	// let root_node = tree.root_node();

	let mut cursor = tree.walk();

	// println!("{:#?}", cursor.);
	rec(&mut cursor);
}

fn rec(c: &mut TreeCursor) {
	println!(
		"{}{:#?}",
		"  ".repeat(c.depth() as usize),
		c.node()
	);

	if c.goto_first_child() {
		rec(c);
	}

	while c.goto_next_sibling() {
		rec(c);
	}
}
