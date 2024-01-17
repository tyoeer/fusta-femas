use std::borrow::Cow;
use bevy_reflect::Reflect;

///Trait for structs that can be used to access a field in an object
pub trait Field {
	///The type we can access a field of
	type Object: ?Sized;
	///The type of the field we're accessing
	type FieldType: ?Sized;
	
	fn name(&self) -> &str;
	fn get<'object>(&self, object: &'object Self::Object) -> &'object Self::FieldType;
	fn get_mut<'object>(&self, object: &'object mut Self::Object) -> &'object mut Self::FieldType;
}


/// dyn [Field] with a static known sized, so they don't have to be boxed
#[derive(Debug)]
pub struct DynField<Object> {
	name: Cow<'static, str>,
	get: fn(&Object) -> &dyn Reflect,
	get_mut: fn(&mut Object) -> &mut dyn Reflect,
}

impl<Object> Clone for DynField<Object> {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			get: self.get,
			get_mut: self.get_mut,
		}
	}
}

impl<Object> DynField<Object> {
	pub const fn new(
		name: Cow<'static, str>,
		get: fn(&Object) -> &dyn Reflect,
		get_mut: fn(&mut Object) -> &mut dyn Reflect,
	) -> Self {
		Self {
			name,
			get,
			get_mut,
		}
	}
}

impl<Object> Field for DynField<Object> {
	type Object = Object;
	type FieldType = dyn Reflect;
	
	fn name(&self) -> &str {
		self.name.as_ref()
	}
	fn get<'object>(&self, object: &'object Self::Object) -> &'object Self::FieldType {
		(self.get)(object)
	}
	fn get_mut<'object>(&self, object: &'object mut Self::Object) -> &'object mut Self::FieldType {
		(self.get_mut)(object)
	}
}

impl<Object> Field for &DynField<Object> {
	type Object = Object;
	type FieldType = dyn Reflect;
	
	fn name(&self) -> &str {
		self.name.as_ref()
	}
	fn get<'object>(&self, object: &'object Self::Object) -> &'object Self::FieldType {
		(self.get)(object)
	}
	fn get_mut<'object>(&self, object: &'object mut Self::Object) -> &'object mut Self::FieldType {
		(self.get_mut)(object)
	}
}

/**
Builds a DynField struct for the given field on the given type

# Example
```
# use ff_object::dyn_field;
# use ff_object::fields::*;
struct TestStruct { test_field: i32 }

let field/*: DynField<TestStruct>*/ = dyn_field!(test_field, TestStruct);
# let field: DynField<TestStruct> = field; // type check

let test = TestStruct { test_field: 2 };
assert_eq!(2, *field.get(&test).downcast_ref::<i32>().unwrap());
```
*/
#[macro_export]
macro_rules! dyn_field {
	($field:ident, $object:ty) => {
		$crate::fields::DynField::<$object>::new(
			std::borrow::Cow::Borrowed(stringify!($field)),
			|obj| &obj.$field,
			|obj| &mut obj.$field,
		)
	};
}

#[cfg(test)]
mod tests {
	use super::*;
	
	struct Test {
		test_field: String
	}
	
	#[test]
	fn dyn_field() {
		let field = DynField::<Test>::new(
			"test".into(),
			|obj| &obj.test_field,
			|obj| &mut obj.test_field,
		);
		
		let test = Test {
			test_field: "test_str".into()
		};
		
		let field_value = field.get(&test).downcast_ref::<String>().unwrap();
		assert_eq!(&test.test_field, field_value)
	}
	
	#[test]
	fn dyn_field_macro() {
		let field = dyn_field!(test_field, Test);
		
		let test = Test {
			test_field: "test_str".into()
		};
		
		let field_value = field.get(&test).downcast_ref::<String>().unwrap();
		assert_eq!(&test.test_field, field_value)
	}
}
