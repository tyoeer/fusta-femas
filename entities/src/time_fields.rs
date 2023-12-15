use std::fmt::Display;

#[cfg(feature = "orm")]
use sea_orm::entity::prelude::*;
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use bevy_reflect::Reflect;

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] = time::macros::format_description!(
	"[year padding:space]-[month padding:zero]-[day padding:zero]"
);
const TIME_FORMAT: &[time::format_description::FormatItem<'_>] = time::macros::format_description!(
	"[hour padding:space]:[minute padding:zero]"
);
const DATE_TIME_FORMAT: &[time::format_description::FormatItem<'_>] = time::macros::format_description!(
	"[year padding:space]-[month padding:zero]-[day padding:zero] [hour padding:space]:[minute padding:zero]:[second padding:zero]"
);

#[derive(
	Clone, Debug, PartialEq, Eq,
	From, Into,
	Serialize, Deserialize,
	Reflect
)]
#[reflect_value]
#[cfg_attr(feature="orm", derive(DeriveValueType) )]
pub struct PrimitiveDateTime(pub time::PrimitiveDateTime);

impl Display for PrimitiveDateTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.format(DATE_TIME_FORMAT).expect("format should be fine"))
	}
}


#[derive(
	Clone, Debug, PartialEq, Eq,
	From, Into,
	Serialize, Deserialize,
	Reflect
)]
#[reflect_value]
#[cfg_attr(feature="orm", derive(DeriveValueType) )]
pub struct Date(pub time::Date);

impl Display for Date {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.format(DATE_FORMAT).expect("format should be fine"))
	}
}


#[derive(
	Clone, Debug, PartialEq, Eq,
	From, Into,
	Serialize, Deserialize,
	Reflect
)]
#[reflect_value]
#[cfg_attr(feature="orm", derive(DeriveValueType) )]
pub struct Time(pub time::Time);

impl Display for Time {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0.format(TIME_FORMAT).expect("format should be fine"))
	}
}


#[derive(
	Clone, Debug, PartialEq, Eq,
	From, Into,
	Serialize, Deserialize,
	Reflect
)]
#[reflect_value]
#[cfg_attr(feature="orm", derive(DeriveValueType) )]
pub struct OptionTime(pub Option<time::Time>);

impl Display for OptionTime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self.0 {
			Some(time) => time.format(TIME_FORMAT).expect("format should be fine"),
			None => "".to_string(),
		};
		write!(f, "{str}")
	}
}