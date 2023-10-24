use box_tt::BoxNew;

#[derive(BoxNew)]
struct Person {
	age: u8,
	extra: Box<usize>,
}

fn main() {
	let person = Person::new_box(30, 123);

	assert_eq!(person.age, 30);
	assert_eq!(person.extra, Box::new(123));
}
