/*!

Server-side filtering stuff.

For a high-level overview of the filter life-cycle, see [app::query](../app/query/index.html).

*/


pub mod shared;

cfg_if::cfg_if! { if #[cfg(feature = "server")] {

pub mod filter;
pub mod filter_list;
///Actual filters
pub mod filters;

}}