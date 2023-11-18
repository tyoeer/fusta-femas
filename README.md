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

### Prereqs

- `cargo install`
	- `sea-orm-cli`
	- `cargo-leptos`

### Database management

- Set up a development database with `sea migrate -d sea-migration fresh`
- Update entities:
	- Update model in [`entities`](/entities/)
	- Add migration in [`sea-migration`](/sea-migration/)
	- You could use the following command to check if the models about match
		- `sea generate entity --lib -o entities/src --with-serde both`
		- It loses a lot of type information though
			- `time::PrimitveDateTime`
			- enums
			- https://github.com/SeaQL/sea-orm/issues/924

## Useful links

- [Leptos book](https://leptos-rs.github.io/leptos/)
- [SeaORM docs](https://www.sea-ql.org/SeaORM/docs/index/)
- [SeaORM cookbook](https://www.sea-ql.org/sea-orm-cookbook/)
- [SeaORM reference](https://docs.rs/sea-orm/latest/sea_orm/)