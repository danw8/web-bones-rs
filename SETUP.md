# Rust Lang Windows setup

The goal of this setup file is to give a good way to work with rust and be able to compile this repository.

## Install Rust

1. Install [Visual Studio](https://www.visualstudio.com/)
..* This is needed for the msvc compiler
..* Make sure to add c++
2. Install [Rustup](https://www.rust-lang.org/en-US/)
3. Check that it works
..* `rustc --version` or `cargo --version`

## Install vscode

1. Download and Install [vscode](https://www.visualstudio.com/)
2. Install rls-vscode extension
..* view -> extensions
..* type `rust`
..* choose Rust (rls) and click install then reload.
3. Install nightly toolchain `rustup install nightly`
4. Install rust src `rustup component add rust-src`
5. Install racer `cargo install racer`

## Optionally install clippy and cargo-update
1. `cargo install clippy`
2. For cargo-update cmake needs to be installed [CMAKE](https://cmake.org/download/)
3. `cargo install cargo-update`

## Install Emscripten

note: git needs to be installed.

1. Download [Emscripten](https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html)
2. Extract the downloaded .zip file to the directory of your choice
3. From the terminal run the following commands
..* `emsdk update`
..* `emsdk install latest`
..* `emsdk activate latest` or `emsdk activate latest --global`
..* `emsdk_env.bat` to activate for the current shell.

## Install Postgres

1. [Download PostgreSQL](https://www.postgresql.org/download/)
2. Run the installer.

## Prepare diesel

1. Add enviroment variable PQ_LIB_DIR = 'path to libpq.dll & libpq.lib'
2. `cargo install diesel_cli --no-default-features --features postgres`
3. `cd data`
4. Create a .env file `echo DATABASE_URL=postgres://webbones:password@localhost/web_bones > .env`
5. `diesel setup`
6. `diesel migration run`

## Create Development TLS key and cert.
