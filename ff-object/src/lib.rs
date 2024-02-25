///Stuff dealing with the fields an object can have
pub mod fields;

mod object_ref;
pub use object_ref::ObjRef;

pub mod describe;

///Traits for representing object behaviour
pub mod traits;
pub use traits::Object;
#[cfg(feature="orm")]
pub use traits::View;



use leptos::{
	SignalWith,
	Signal,
	IntoSignal,
};
use Object as ObjectTrait;

///Returns a [Signal] containing an ObjRef for the given signal containing an object
pub fn ref_signal<Object: ObjectTrait>(object: impl SignalWith<Value=Object> + 'static) -> Signal<ObjRef<Object>> {
	let closure = move || object.with(|object| object.get_ref());
	closure.into_signal()
}

