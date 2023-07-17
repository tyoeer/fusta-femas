# Fusta Femas

Full Stack Feed Management System

Aims to not only collect feeds into a database, but also provide convenient ways to consume/read them.

## Getting started

### Prereqs 
- `cargo install`
	- `sea-orm-cli`

- Set up a development database with `sea migrate -d sea-migration fresh`
- Update entities with `sea generate entity --lib -o sea-entities/src`
	- https://github.com/SeaQL/sea-orm/issues/924