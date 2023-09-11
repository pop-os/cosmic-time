check *args: (check-cosmic args) (check-iced args)

check-cosmic *args:
    cargo clippy --no-default-features --features wayland-libcosmic {{args}} -- -W clippy::pedantic
    cargo clippy --no-default-features --features winit-libcosmic {{args}} -- -W clippy::pedantic

check-iced *args:
    cargo clippy --no-default-features --features iced {{args}} -- -W clippy::pedantic

check-json: (check '--message-format=json')

# Auto-apply recommend clippy fixes, and format code.
fix: (check "--fix --allow-dirty --allow-staged")
    @cargo fmt
