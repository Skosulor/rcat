# rcat: cat rewritten in rust

Goal: rewrite cat from GNU's coreutils in rust.

Why? To learn rust and programming.

Will it do anything different or better? Probably not.

## Installion

requirements: [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html "cargo")


```
git clone https://github.com/Skosulor/rcat
cd rcat
cargo build --release
```

path to rcat: `rcat/target/release/rcat`

## Usage

`rcat [FLAGS] [FILE]`

if no file is given, input is taken from `stdin` 

## Implemented features

* [X] Read file 
* [X] print file 
* [X] Read from stdin and print
* [X] Handle input error
* [X] Options
  * [X] -A, --show-all equivalent to -vET
  * [X] -b, --number-nonblank
  * [X] -e, equivalent to -vE
  * [X] -E, --show-ends
  * [X] -n, --number
  * [X] -s, --squeeze blank
  * [X] -t  equivalent to -vT
  * [X] -T, --show-tabs
  * [X] -u (ignored)
  * [X] -v, --show-nonprinting
  * [X] --help display this help and exit
  * [X] --version
* [X] Write help
