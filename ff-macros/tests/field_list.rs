use std::fmt::Debug;
use bevy_reflect::Reflect;

use ff_object::fields::{FieldListable, Field};

const FAILURES_PATH: &str = "tests/field_list_failures/*.rs";

use ff_macros::FieldList;

#[derive(FieldList, Default)]
struct Test {
	field_one: i32,
	other_field: bool,
}



#[test]
fn works() {
	fn test_field<FieldType: Default + PartialEq + Debug + Reflect + Clone>(test: &mut Test, field_name: &str, new_value: FieldType) {
		let field = Test::iter_fields()
			.find(|field| field.name()==field_name)
			.expect("field should be listed");
		let value = field.get(test).downcast_ref::<FieldType>().expect("field should be of the right type");
		assert_eq!(FieldType::default(), *value);
		
		let mut_value = field.get_mut(test).downcast_mut::<FieldType>().expect("field should be of the right type");
		*mut_value = new_value.clone();
		
		let value = field.get(test).downcast_ref::<FieldType>().expect("field should be of the right type");
		assert_eq!(new_value, *value);
	}
	
	let mut test = Test::default();
	
	assert_eq!(2, Test::iter_fields().count());
	
	test_field(&mut test, "field_one", 42);
	test_field(&mut test, "other_field", true);
}

#[test]
fn unit() {
	#[derive(FieldList, Default)]
	struct TestUnit;
	
	assert_eq!(0, TestUnit::iter_fields().count())
}

#[test]
fn failures() {
	let t = trybuild::TestCases::new();
	t.compile_fail(FAILURES_PATH);
}