set windows-shell := ["cmd.exe", "/c"]

# cargo binstalls needed development tools
binstall:
	cargo binstall cargo-leptos
	cargo binstall sea-orm-cli

# DANGEROUS Rebuilds the database from migrations
[confirm]
reset-db:
	sea migrate -d sea-migration fresh

# cargo leptos watch
watch:
	cargo leptos watch --hot-reload