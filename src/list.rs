fn main() {
 	let last = Element {value : 0, next_element: None};
 	let first = Element {value : 1, next_element: Some(Box::new(last))};
 	println!("{}", first.value);
 	let a = first.next_element.unwrap();
 	println!("{}", a.value);
}

struct Element {
	value: i32,
	next_element: Option<Box<Element>>
}