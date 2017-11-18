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

## Create Development TLS key and cert.
