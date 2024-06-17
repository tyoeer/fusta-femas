use ff_macros::*;

#[derive(FieldList)]
#[field_list(list(dyn ToString))]
struct WrongPropertyName;

#[derive(FieldList)]
#[field_list(lists)]
struct NoContents;

#[derive(FieldList)]
#[fieldlist(lists)]
struct WrongAttributeName;

#[derive(FieldList)]
#[field_list(test(test))]
struct CompletelyWrong;

fn main() {}