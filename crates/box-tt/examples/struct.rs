use box_tt::BoxNew;

#[derive(BoxNew)]
struct Person {
	age: u8,
	extra: Box<usize>,
}

fn main() {
	// let person = Person
}