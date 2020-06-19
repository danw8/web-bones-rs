# Rust Lang Windows setup

The goal of this setup file is to give a good way to work with rust and be able to compile this repository.

## Install Rust

1. Install [Visual Studio](https://www.visualstudio.com/).
  * This is needed for the msvc compiler.
  * Make sure **C++** support is added.
  * A library used by Rocket.rs requires VS2015 v3 or later but does not support 2017 (yet?)
2. Install [Rustup](https://www.rust-lang.org/en-US/).
3. Check that it works.
  * `rustc --version` or `cargo --version`.

## Install vscode

1. Download and Install [vscode](https://www.visualstudio.com/).
2. Install rls-vscode extension.
  * view -> extensions.
  * type `rust`.
  * choose Rust (rls) and click install then reload.
3. Optionally install the rls manually.
  * `rustup component add rls-preview --toolchain nightly`.
4. Install nightly toolchain `rustup install nightly`.
5. Install rust analysis `rustup component add rust-analysis`.
6. Install rust src `rustup component add rust-src`.
7. Install racer `cargo install racer`.

## Optionally install clippy and cargo-update
1. `cargo install clippy`.
2. For cargo-update cmake needs to be installed [CMAKE](https://cmake.org/download/).
3. `cargo install cargo-update`.
  * This may fail ... Just move on. (broken at time of writing).

## Install Postgres

1. [Download PostgreSQL](https://www.postgresql.org/download/).
2. Run the installer.
3. Use PgAdmin to create a user that can create databases.
4. You need to add `libpq.dll` to your PATH it is located in the bin directory of the postgres install. 

## Prepare diesel

1. Add enviroment variable PQ_LIB_DIR = path to libpq.dll & libpq.lib (ex. C:\Program Files\PostgreSQL\9.6\lib).
2. You may need to also add the postgres bin and lib directory to the PATH environment variable.
  * (ex. C:\Program Files\PostgreSQL\9.6\lib)
  * (ex. C:\Program Files\PostgreSQL\9.6\bin)
3. `cargo install diesel_cli --no-default-features --features postgres`.
4. If you haven't clone the rep
  * `git clone https://github.com/danw8/web-bones-rs`
5. `cd web-bones-rs/data`.
6. Create a .env file `echo DATABASE_URL=postgres://user:password@localhost/web_bones > .env`.
  * Replace the information with that of the user you created in postgres.
  * .env doesn't seem to be working anymore so you might have to create the environment variable manually.
7. `diesel setup`.
8. `diesel migration run`.

## Create Development TLS key and cert.

1. Skip this unless you need your development environment secure.
2. (not implemented).

## Build the server

1. `rustup default nightly` if you are on a different toolchain.
2. `cargo build` in the root directory.
  * If build fails you may need to add the DATABASE_URL as an enviroment variable.

## Install Emscripten (Optional: Use the rust target)

note: git needs to be installed.

1. Download [Emscripten](https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html).
2. Extract the downloaded .zip file to the directory of your choice.
3. From the terminal run the following commands.
  * `emsdk update`.
  * `emsdk install latest`.
  * `emsdk activate latest` or `emsdk activate latest --global`.
  * `emsdk_env.bat` to activate for the current shell.
4. `rustup default nightly`
5. `rustup target add wasm32-unknown-emscripten`.
6. `rustup target add asmjs-unknown-emscripten`.

## Native Rust wasm target

1. `rustup target add wasm32-unknown-unknown`
2. install cargo web `cargo install -f wasm-pack`

## Build the frontend

#### Rust native target

1. `cp app`
2. `wasm-pack build --target web --out-name app --out-dir ../assets/`


## Run the application

1. `rustup default nightly` if you are on a different toolchain.
2. `cargo run -p web` from the root directory.


## Documentation Issues

Please feel free to open issue about this repository.
