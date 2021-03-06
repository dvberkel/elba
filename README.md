# elba

[![Not-Windows Build Status](https://travis-ci.com/elba/elba.svg?branch=master)](https://travis-ci.com/elba/elba) [![Windows Build Status](https://ci.appveyor.com/api/projects/status/j2pk9krx63o1dpdv?svg=true)](https://ci.appveyor.com/project/dcao/elba)

A modern and (hopefully!) fast package manager for Idris.

## Installation

There are three options for installing elba:

1. Download the pre-built binary corresponding to your platform from GitHub Releases and place it in your PATH.
2. Install the package from Rust's crates.io repository using `cargo +nightly install elba`
3. Manually build and install elba yourself using the source code with `git clone https://github.com/elba/elba.git && cd elba && cargo +nightly install`.

## Documentation

[The elba Guide](https://elba.github.io/elba) is intended to be the ultimate source of information on using elba and understanding its functionality.

Documentation for elba-the-Rust-library is hosted at [docs.rs/elba](https://docs.rs/elba).

## License

elba itself is distributed under the [MIT License](./LICENSE).