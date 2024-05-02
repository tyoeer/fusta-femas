/*!

[ffilter::filter::Filter]s live on the server in a [ffilter::filter_list::FilterList].

When we want them on the client, we turn them into a [filter::FilterDesc],
using [ffilter::filter::DynReprArgument] (which is a dyn wrapper around [ffilter::filter::ReprArgument])
and [ff_object::describe::DynDescribe] (which is a dyn wrapper around [ff_object::describe::Describe])
in [filter::get_filters].


In order to edit/configure on of those, it gets turned into a [ClientFilter] in [ClientFilter::from_description],
which has Leptos signals all over its internals,
which then can get edited using a [FilterUI].


In order to get the server to do things again, the [ClientFilter] gets turned into a [Filter], which is designed for transport.
On the server, this one can be converted into a [`Box`]`< dyn `[`ffilter::filter::Filter`]`>` using [Filter::into_filter],
which can then filter a query with [ffilter::filter::Filter::filter].
All this currently/at the time of this writing has to be done by the end user, e.g. [crate::feeds::search::search2].


*/


pub mod filter;
pub use filter::{
	FilterUI,
	ClientFilter,
	Filter,
};

pub mod query;
pub use query::{
	QueryUI,
	ClientQuery,
	Query,
	QueryString,
};
