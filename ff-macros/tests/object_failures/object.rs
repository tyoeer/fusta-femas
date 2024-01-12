use ff_object::Object;

const FAILURES_PATH: &str = "tests/object_failures/*.rs";

use ff_macros::Object;

#[derive(Object, Default)]
pub struct Model {
	id: i32,
}

mod test {
	use ff_macros::Object;
	
	#[derive(Object, Default)]
	pub struct Model {
		id: i32,
	}
	
	pub mod more_test {
		use ff_macros::Object;
		
		#[derive(Object, Default)]
		pub struct Model {
			id: i32,
		}
	}
}


#[test]
fn compiles() {
	fn test<Obj: Object + Default>(str: &str) {
		let test = Obj::default();
		assert_eq!(0, test.get_id());
		assert_eq!(str, Obj::get_object_name());
	}
	
	test::<Model>("object");
	test::<test::Model>("test");
	test::<test::more_test::Model>("more_test");	
}

#[test]
fn failures() {
	let t = trybuild::TestCases::new();
	t.compile_fail(FAILURES_PATH);
}