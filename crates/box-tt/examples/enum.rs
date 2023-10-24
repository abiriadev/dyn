use box_tt::BoxNew;

#[derive(BoxNew)]
enum Error {
	UnknownError,
	PathReadError(Box<String>),
}

fn main() {
	let person = Person::new_box(30, 123);

	assert_eq!(person.age, 30);
	assert_eq!(person.extra, Box::new(123));
}
