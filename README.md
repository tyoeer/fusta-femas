# Fusta Femas

Full Stack Feed Management System

Aims to not only collect feeds into a database, but also provide convenient ways to consume/read them.

## Getting started

### Prereqs

- `cargo install`
	- `sea-orm-cli`
	- `cargo-leptos`
		- https://github.com/leptos-rs/cargo-leptos/pull/159

### Database management

- Set up a development database with `sea migrate -d sea-migration fresh`
- Update entities:
	- `sea generate entity --lib -o sea-entities/src --with-serde both`
		- https://github.com/SeaQL/sea-orm/issues/924
	- Then manually set `TimeDateTime` and enums

## Useful links

- [Leptos book](https://leptos-rs.github.io/leptos/)
- [SeaORM docs](https://www.sea-ql.org/SeaORM/docs/index/)
- [SeaORM cookbook](https://www.sea-ql.org/sea-orm-cookbook/)
- [SeaORM reference](https://docs.rs/sea-orm/latest/sea_orm/)