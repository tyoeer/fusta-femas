pub trait Object {
	fn get_id(&self) -> i32;
	fn get_object_name() -> &'static str where Self: Sized;
}