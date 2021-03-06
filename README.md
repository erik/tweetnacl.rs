# knuckle [![Build Status](https://travis-ci.org/erik/knuckle.svg?branch=master)](https://travis-ci.org/erik/knuckle)

**WARNING:** This library is not ready for production! It should be treated as
  insecure until proven otherwise.

Knuckle is a library designed to make common cryptography operations in Rust as
easy and secure as possible. It does this by hiding all unnecessary
implementation details, so rather than knowing you're using the
[Salsa20 cipher](http://en.wikipedia.org/wiki/Salsa20), you just need to use a
`secretbox` to perform symmetric key encryption.

Knuckle is built on top of the NaCl library, which has the same goal of being a
sort of cryptographic black box. Knuckle provides direct access to the NaCl
API, as well as a more Rust-friendly interface.

[Check out the documentation for usage information.](http://rust-ci.org/erik/knuckle/doc/knuckle/)

## Building

You'll need [Cargo](http://crates.io/) to build knuckle. After you get that set up:

```bash
$ cargo build
$ cargo test
```

To include knuckle in your project, add the following to your `Cargo.toml`:

```toml
[dependencies.knuckle]
git = "https://github.com/erik/knuckle"
```

## License

knuckle is distributed under the MIT license. See the `LICENSE` file in this
directory for more information.
