set windows-shell := ["cmd.exe", "/c"]

# cargo binstalls needed development tools
binstall:
	cargo binstall cargo-leptos
	cargo binstall sea-orm-cli


# cargo leptos watch
watch:
	cargo leptos watch --hot-reload
# cargo leptos serve
up:
	cargo leptos serve
# cargo leptos serve
build:
	cargo leptos build


# Serve the alt database
alt $FUSTA_FEMAS_DATA_PATH=".local-ff-data/alt/":
	cargo leptos serve
