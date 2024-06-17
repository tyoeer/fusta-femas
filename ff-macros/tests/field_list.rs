use std::fmt::Debug;
use bevy_reflect::Reflect;

use ff_object::fields::{Field, FieldListable};

const FAILURES_PATH: &str = "tests/field_list_failures/*.rs";

use ff_macros::FieldList;




fn test_field_reflect<
	Object: FieldListable<dyn Reflect>,
	FieldType: Default + PartialEq + Debug + Reflect + Clone
>(test: &mut Object, field_name: &str, new_value: FieldType) {
	let field = Object::iter_fields()
		.find(|field| field.name()==field_name)
		.expect("field should be listed");
	let value = field.get(test).downcast_ref::<FieldType>().expect("field should be of the right type");
	assert_eq!(FieldType::default(), *value);
	
	let mut_value = field.get_mut(test).downcast_mut::<FieldType>().expect("field should be of the right type");
	*mut_value = new_value.clone();
	
	let value = field.get(test).downcast_ref::<FieldType>().expect("field should be of the right type");
	assert_eq!(new_value, *value);
}

#[test]
fn works_reflect() {
	#[derive(FieldList, Default)]
	struct Test {
		field_one: i32,
		other_field: bool,
	}
	
	let mut test = Test::default();
	
	assert_eq!(2, Test::iter_fields().count());
	
	test_field_reflect(&mut test, "field_one", 42);
	test_field_reflect(&mut test, "other_field", true);
}

#[test]
fn multiple_lists() {
	#[derive(FieldList, Default)]
	#[field_list(lists(dyn Reflect, dyn ToString))]
	struct Test2 {
		field_one: i32,
		other_field: bool,
	}
	
	let mut test = Test2::default();
	
	assert_eq!(2, <Test2 as FieldListable<dyn Reflect>>::iter_fields().count());
	assert_eq!(2, <Test2 as FieldListable<dyn ToString>>::iter_fields().count());
	
	test_field_reflect(&mut test, "field_one", 42);
	test_field_reflect(&mut test, "other_field", true);
	
	fn test_string(object: &Test2, field_name: &str, expected: &str) {
		let field = <Test2 as FieldListable<dyn ToString>>::iter_fields()
			.find(|field| field.name()==field_name)
			.expect("field should be listed");
		
		let found = field.get(object).to_string();
		assert_eq!(found, expected);
	}
	
	test_string(&test, "field_one", "42");
	test_string(&test, "other_field", "true");
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