use super::Field;

pub trait FieldListable<FieldType: ?Sized> {
	fn iter_fields() -> impl Iterator<Item = impl Field<Object=Self, FieldType=FieldType>>;
}