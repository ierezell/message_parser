# Install

### Install the correct target (web assembly)

rustup target add wasm32-unknown-unknown

### Install the bundler

cargo install trunk

### Bundle all the code and serve a hot reloaded server

trunk serve --open

### Misc

On windows you might need to add the folder as an exclusion to the antivirus
