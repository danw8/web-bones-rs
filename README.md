# web-bones-rs
This is a repository to get you stared with web development in pure rust.

## Setup
Follow the Setup.md file. This document helps you setup a environment and tools to get you started with rust development.

## Purpose
This repository is meant to be helpful in figuring out how differnt libraries in the rust ecosystem that can be used to build web apps in rust.
Some of the libraries this repository uses are:

1. [Rocket](https://rocket.rs/)
2. [Deisel](diesel.rs)
3. [Yew](https://github.com/DenisKolodin/yew)

## Server Side
This server is built on top of the Rocket web framework with Diesel ORM connected to Postgres as the data layer. Maud is used as a type safe templating engine.

## Client Side (WIP)
This client is built with Yew a rust to wasm front end library.
