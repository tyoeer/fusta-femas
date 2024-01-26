# Fusta Femas

A very very WIP Full Stack Feed Management System prototype.

## Goals

### Novelty

- Streamlined consumption
	- In-browser system with easy next button
		- With all your addons and configuration
		- Preloading
	- Custom filtering and sorting
	- The ability to have different consumption feeds
- Reporting
	- Make it easy to figure out what went wrong when a fetch failed
	- Supply an overview of feeds that stopped producing content / slowed down
	- Report how much content is produced and consumed

### Basics

- Desktop program
- Acquire feeds from different types of sources

## Getting started

### Tools

#### Basically required

- `cargo-leptos`
- `sea-orm-cli`

#### Other

[There's a `.justfile` to run shortcuts with `just`](https://just.systems).

Run `just binstall` to install the requirements with [`cargo binstall`](https://github.com/cargo-bins/cargo-binstall)

### Database management

- Update entities:
	- Update model in [`entities`](/entities/)
	- Add migration in [`sea-migration`](/sea-migration/)
		- `sea migrate -d sea-migration generate [[NAME]]`
		- Add the new migration in `migrations()` in [`sea-migration/src/lib.rs`](sea-migration/src/lib.rs)
	- In case of new model:
		- Add it in the entities prelude
	- Many to many relations:
		```rust
		impl Related<to::Entity> for from::Entity {
			fn to() -> RelationDef {
				between::Relation::To.def()
			}
			fn via() -> Option<RelationDef> {
				Some(between::Relation::From.def().rev())
			}
		}
		```
	- You could use the following command to check if the models about match
		- `sea generate entity --lib -o entities/src --with-serde both`
		- It loses a lot of type information though
			- `time::PrimitveDateTime`
			- enums
			- https://github.com/SeaQL/sea-orm/issues/924
- You can reset the database by removing all files starting with `content.db`, Fusta Femas will recreate it on startup
- When using `sea migrate` subcommands that connect to the database, don't forget to set the `DATABASE_URL` environment variable to `.local-ff-data/[profile]/content.db`

### Alt database

I wanted to use Fusta Femas already while still developing it, before building a whole release and distribution pipeline/process.

To serve the current codebase with the alt database, run `just alt`.

To reset the database, run `just reset-alt-db`

## Useful links

- [Leptos book](https://leptos-rs.github.io/leptos/)
- [SeaORM docs](https://www.sea-ql.org/SeaORM/docs/index/)
- [SeaORM cookbook](https://www.sea-ql.org/sea-orm-cookbook/)
- [SeaORM reference](https://docs.rs/sea-orm/latest/sea_orm/)