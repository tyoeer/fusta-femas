use super::Field;

///Trait for structs that can have their fields listed
pub trait FieldListable<FieldType: ?Sized> {
	//'statics required for current implementation of iter_field_values
	// the field name borrows the field, and needs to live long enough
	fn iter_fields() -> impl Iterator<Item = &'static (impl Field<Object=Self, FieldType=FieldType> + 'static)>;
	
	fn iter_values<'this>(&'this self) -> impl Iterator<Item = &'this FieldType> where FieldType: 'this {
		Self::iter_fields().map(|field| field.get(self))
	}
	
	fn iter_field_values<'this>(&'this self) -> impl Iterator<Item = (&'static str, &'this FieldType)> where FieldType: 'this {
		Self::iter_fields().map(move |field | {
			(field.name(), field.get(self))
		})
	}
}