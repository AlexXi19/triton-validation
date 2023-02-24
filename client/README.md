# Validation Frontend Client

## Initializing your workspace

### Install cargo-leptos

`cargo install --locked cargo-leptos`

### Installing Tailwind

You can install Tailwind using `npm`:

```bash
npm install -D tailwindcss
```

If you'd rather not use `npm`, you can install the Tailwind binary [here](https://github.com/tailwindlabs/tailwindcss/releases).

## Install Tooling

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5. `npm install -g sass` - install `dart-sass` (should be optional in future

## Dev

`npx tailwindcss -i ./input.css -o ./style/output.css --watch`

and

`cargo leptos watch`

in this directory.

Open browser on [http://localhost:3000/](http://localhost:3000/)

You can begin editing your app at `src/app.rs`.

### Server Side Rendering With Hydration

To run it as a server side app with hydration, first you should run

```bash
wasm-pack build --target=web --no-default-features --features=hydrate
```

to generate the WebAssembly to hydrate the HTML delivered from the server.

Then run the server with `cargo run` to serve the server side rendered HTML and the WASM bundle for hydration.

```bash
cargo run --no-default-features --features=ssr
```

> Note that if your hydration code changes, you will have to rerun the wasm-pack command above before running
> `cargo run`

### Client Side Rendering

You'll need to install trunk to client side render this bundle.

1. `cargo install trunk`
   Then the site can be served with `trunk serve --open`
