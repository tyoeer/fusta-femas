use ff_macros::*;

struct NotReflect;

#[derive(FieldList)]
struct Test {
	field: NotReflect,
}

fn main() {}