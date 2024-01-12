pub trait Object {
	fn get_id(&self) -> i32;
	fn get_object_name() -> &'static str where Self: Sized;
}

impl<T> Object for T where T: ff_object::Object {
	fn get_id(&self) -> i32 {
		self.get_id()
	}
	fn get_object_name() -> &'static str where Self: Sized {
		Self::get_object_name()
	}
}