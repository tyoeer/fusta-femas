# Used frameworks / dependencies

Fusta Femas is written in [the Rust language](https://rust-lang.org), and is organised into multiple crates in a [cargo workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html.)

## UI / Leptos

Fusta Femas is build using/on [the Leptos framework](https://leptos.dev) for UI. This allows us to render our UI directly in the browser, while also managing communication with the not-browser part.

This leads to two overlapping modules:
- The server, which runs as a separate program, and does all the feed management stuff. \
Also referred to as the "binary".
- The client, which runs in the browser and (hopefully) speeds up the UI by not requiring a a round trip to the server every time something happens. \
Also referred to as the "lib"rary, because it has no `main()`.

Fusta Femas also uses server-side rendering (ssr), which also renders pages on the server to (hopefully) speed up the first page load.

In order to more easily work with a system that needs two compiled packages, there is [cargo leptos](https://github.com/leptos-rs/cargo-leptos).

## Data / SeaORM

Fusta Femas stores its feed related data in a database, and uses [SeaORM](https://www.sea-ql.org/SeaORM/) to give us a high-level interface with it.

# Crates

In rough order in which they depend on each other:

- [`ff-macros/`](ff-macros/): [Rust](https://rust-lang.org) requires all macros to live in a separate crate. This is that crate.
- [`ff-object/`](ff-object/): More generic stuff that could be independent of Fusta Femas. Mostly "object" related functionality.
- [`entities/`](entities/): structs representing the data in the database + some utility stuff for those.
- [`sea-migration/`](sea-migration/): Runnable migrations to make the database match the entities crate. Server only.
- [`acquire/`](acquire/): Anything that has to do with getting feed entries from the internet into our database. Server only.
- [`ffilter/`](ffilter/): Anything that has to do with filtering feeds & entries. Server only.
- [`app/`](app/): Has all the UI stuff. Shared for server & client.
- [`server-setup/`](server-setup/): Contains setup and boilerplate for server specific stuff.
- [`server-entrypoint/`](server-entrypoint/): Hooks up [`server-setup/`](server-setup/) and [`app/`](app/) together, and sets up the available strategies and filters.
- [`client-entrypoint/`](client-entrypoint/): Sets up logging and hooks up [`app/`](app/) in the client.


# (Tool) Config

- [`.github/`](.github/): Everything related to [GitHub](https://github.com).
	- [`dependabot.yml`](.github/dependabot.yml): Configuration to automatically open PRs to update dependencies using [Dependabot](https://github.com/dependabot).
- [`.justfile`](.justfile): Shortcuts/commands using [Just](https://just.systems) to ease development.
- [`Cargo.toml`](Cargo.toml): Lists crates, dependencies, and [cargo leptos](https://github.com/leptos-rs/cargo-leptos) configuration.
- [`Cargo.lock`](Cargo.lock): Exact versions of the dependencies we're using for deterministic and reproducible builds.

# Docs

- [`ARCHITECTURE.md`](ARCHITECTURE.md): High level overview of Fusta Femas and everything in this repository. If you want to find which code is responsible for something, start here.
- [`README.md`](README.md): Project introduction and miscellaneous info dump.
- [`InitialDesign.md`](InitialDesign.md): Original idea and design sketch. Can be removed after tags are done.
- [`TagsDesign.md`](TagsDesign.md): Design notes for tags.

# Dev storage

- [`.local-ff-data/`](.local-ff-data/): Folder to contain ff data that for during development
	- [`dev/`](.local-ff-data/dev/): Stuff for during normal development.
	- [`alt/`](.local-ff-data/alt/): Stuff for [the alt database](README.md#alt-database)