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
# cargo leptos serve
up:
	cargo leptos serve

# Serve the alt database
alt $DATABASE_URL="sqlite://.local/alt/db/alt.db?mode=rwc":
	cargo leptos serve

# DANGEROUS Rebuilds the database from migrations
[confirm]
reset-alt-db $DATABASE_URL="sqlite://.local/alt/db/alt.db?mode=rwc":
	sea migrate -d sea-migration fresh