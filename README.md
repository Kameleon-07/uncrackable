# uncrackable

## Description

Uncrackable is a simple tool to generate secure passwords. Password randomization can be personalized with parameters like:
* Password length
* Usage of special characters
* Usage of upper case letters
And more.

## Motivation

I made this project to practice what I've learned about Rust, and to learn new things like simple GUI programming with egui library.

## Requirements

* any linux distro (tested on ArchLinux, don't know if works on other systems)
* rust compiler

## Installation

```
git clone https://github.com/Kameleon-07/uncrackable.git
cd rstrings
cargo install --path .
strip ~/.cargo/bin/uncrackable
```

#### Or more manually

```
git clone https://github.com/Kameleon-07/uncrackable.git
cd uncrackable
cargo build --release
(optional but recommended) strip target/release/uncrackable
sudo cp target/release/uncrackable /usr/local/bin
```