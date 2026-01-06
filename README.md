# leptos-termux
install and use leptos in termux

## depends 
- install these pkgs in termux 

```
apt install rust trunk rust-std-wasm32-unknown-unknown
cargo install wasm-bindgen-cli 

# Create the symlink manually
ln -sf ~/.cargo/bin/wasm-bindgen $PREFIX/bin/wasm-bindgen
```

## getting started 
- create a basic Rust project
```
cargo init leptos-tutorial
```
- cd into your new leptos-tutorial project and add leptos as a dependency
```
cargo add leptos --features=csr
```

- Create a simple index.html in the root of the leptos-tutorial directory

```
<!DOCTYPE html>
<html>
  <head></head>
  <body></body>
</html>
```

- And add a simple “Hello, world!” to your main.rs
```
use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| view! { <p>"Hello, world!"</p> })
}
```

Now run trunk `serve --open` from the root of the leptos-tutorial directory. Trunk should automatically compile your app and open it in your default browser. If you make edits to main.rs, Trunk will recompile your source code and live-reload the page.

Welcome to the world of UI development with Rust and WebAssembly (WASM), powered by Leptos and Trunk!
