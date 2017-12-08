# web-bones-rs
This is a repository to get you stared with web development in pure rust.

## Setup
Follow the Setup.md file. This document helps you setup a environment and tools to get you started with rust development.

## Purpose
This repository is meant to be helpful in figuring out how differnt libraries in the rus eco system can be used to build web apps in rust.
Some of the libraries this repository uses are:

1. [Rocket](https://rocket.rs/)
2. [Deisel](diesel.rs)
3. [Maud](https://maud.lambda.xyz/)
4. [STDWEB](https://github.com/koute/stdweb)
5. and more to come...

## Server Side
This server is built on top of the Rocket web framework with Diesel ORM connected to Postgres as the data layer. Muad is used as a type safe templating engine.

## Client Side (WIP)
This client is built on top of the stdweb library and uses maud for templating when it can.
