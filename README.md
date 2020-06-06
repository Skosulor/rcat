# rcat: cat rewritten in rust [![Build status](https://travis-ci.org/Skosulor/rcat.svg)](https://travis-ci.org/skosulor/rcat)

Goal: rewrite cat from GNU's coreutils in rust.

Why? To learn rust and programming.

Will it do anything different or better? Probably not.

## Installation

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


## Differences from cat

* [ ] TODO: cannot open any file, e.g. opening a binary file produces an error.
* [ ] TODO: with flag 'non-printing' non-ascii characters are printed as '^?'
      instead of the control sequence.
* If show-ends and number-nonblank flags are set the '$' sign in blank lines will
have the same line start as numbered lines. This shall be kept as an feature as
it looks neater.



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
