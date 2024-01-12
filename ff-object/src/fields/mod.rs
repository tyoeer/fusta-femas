///The stuff for the representing the actual fields
mod core;
pub use core::Field;
pub use core::DynField;

///Stuff relating to the objects that have the fields
mod object;
pub use object::FieldListable;