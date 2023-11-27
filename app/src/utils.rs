use leptos::*;
use leptos_router::{use_params, Params, IntoParam, ParamsError};

#[derive(leptos::Params, Clone, PartialEq, Eq)]
struct IdParam {
	// Has to be an Option due to Leptos limitations
	pub id: Option<i32>,
}

pub fn with_id_param<Mapped>(callback: impl Fn(i32) -> Mapped) -> Result<Mapped, ParamsError> {
	let id_res_memo = use_params::<IdParam>();
	let res = id_res_memo.get().map(|params| params.id);
	
	match res {
		Ok(Some(id)) => Ok(callback(id)),
		Ok(None) => Err(ParamsError::MissingParam("id".to_string())),
		Err(err) => Err(err),
	}
	
}