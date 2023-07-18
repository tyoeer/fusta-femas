# Fusta Femas

Full Stack Feed Management System

Aims to not only collect feeds into a database, but also provide convenient ways to consume/read them.

## Getting started

### Prereqs 
- `cargo install`
	- `sea-orm-cli`
	- `cargo-leptos`
		- https://github.com/leptos-rs/cargo-leptos/pull/159

- Set up a development database with `sea migrate -d sea-migration fresh`
- Update entities with `sea generate entity --lib -o sea-entities/src --with-serde`
	- https://github.com/SeaQL/sea-orm/issues/924

## Useful links

- [Leptos book](https://leptos-rs.github.io/leptos/)
- [SeaORM docs](https://www.sea-ql.org/SeaORM/docs/index/)
- [SeaORM cookbook](https://www.sea-ql.org/sea-orm-cookbook/)
- [SeaORM reference](https://docs.rs/sea-orm/latest/sea_orm/)