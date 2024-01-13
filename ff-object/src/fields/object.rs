use super::Field;

///Trait for structs that can have their fields listed
pub trait FieldListable<FieldType: ?Sized> {
	fn iter_fields() -> impl Iterator<Item = impl Field<Object=Self, FieldType=FieldType>>;
}