# <img src="https://github.com/whatisaphone/rlbot-rust/raw/master/assets/logo.png" height="128" /> rlbot-rust

[![crates.io](https://img.shields.io/crates/v/rlbot.svg)](https://crates.io/crates/rlbot)
[![docs](https://docs.rs/rlbot/badge.svg)](https://docs.rs/rlbot/)
[![Build Status](https://travis-ci.org/rlbot/rlbot-rust.svg?branch=master)](https://travis-ci.org/rlbot/rlbot-rust)

[RLBot] is a framework for creating offline Rocket League bots. This crate lets
you write bots using a simple, safe interface that should feel comfortable to
Rust developers.

**Documentation:** [We have it.](https://docs.rs/rlbot/)

**Stability:** As you might notice, we're still on version 0.x. Breaking changes
are still possible at this stage. Join the [Discord] to keep up-to-date!

**Compatibility**: We target the latest version of RLBot, and the latest stable
version of Rust.

[RLBot]: https://github.com/RLBot/RLBot
[Discord]: https://discordapp.com/invite/XhrQGf

## Usage

Your code will look a little something like this:

```rust
use rlbot::ffi;

fn main() -> Result<(), Box<Error>> {
    rlbot::run_bot(MyBot { /* ... */ })
}

struct MyBot { /* ... */ }

impl rlbot::Bot for MyBot {
    fn tick(&mut self, packet: &ffi::LiveDataPacket) -> ffi::PlayerInput {
        // ...
    }
}
```

This library comes with plenty of examples to get you started. For a list of
examples, check out the [docs].

[docs]: https://docs.rs/rlbot/

### Installing the framework

RLBot is needed to use this RLBot binding, of course. If the framework is not
found in any of Windows's [DLL search locations], `init()` will return this
error:

[DLL search locations]: https://docs.microsoft.com/en-us/windows/desktop/dlls/dynamic-link-library-search-order#standard-search-order-for-desktop-applications

```text
Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
```

RLBot is written in Python, so the easiest way to get the needed files is to use
`pip`:

```sh
pip install rlbot
```

Then add them to your `PATH` (adapt this command to the location of `rlbot` on
your particular system:

```sh
export PATH="$PATH":/c/Python36/Lib/site-packages/rlbot/dll
```

## Development

### Prerequisites

* Use [rustup] to install both stable Rust and nightly Rust.

  This library targets stable Rust, however nightly Rust is needed for a few
  development niceties like `cargo fmt` and `cargo doc`.

* Install [pre-commit], and run this command:

  ```sh
  pre-commit install
  ```

  This makes it much less likely that your commits will break the build.

[rustup]: https://rustup.rs/
[pre-commit]: https://pre-commit.com/

### Testing

Many of the tests require a copy of Rocket League. All such tests will have
`integration` in their name. Because these tests cannot be run in CI, they
should be run manually before cutting a release, using this command:

```sh
cargo test -- integration
```

### How to compile the flatbuffer schema

Flatbuffers comes with a schema compiler, flatc. Unless your package manager
has flatc and allows building HEAD, you'll have to [build flatc] yourself.

Get the most recent [flatbuffer schema]. Then compile the schema like so from
this project's root:

```sh
flatc -o src --rust rlbot.fbs
cargo fix --allow-dirty
cargo +nightly fmt
```

This will update the `src/rlbot_generated.rs` file.

[build flatc]: https://google.github.io/flatbuffers/flatbuffers_guide_building.html
[flatbuffer schema]: https://github.com/RLBot/RLBot/blob/master/src/main/flatbuffers/rlbot.fbs

### How to generate ffi bindings

Bindings are generated with [rust-bindgen]. Those docs are required reading.

[rust-bindgen]: https://rust-lang.github.io/rust-bindgen/

After bindgen and its prerequisites are installed and working, run:

```sh
make RLBOT_DIR="<path-to-rlbot>" ffi
```

Setting `RLBOT_DIR` is optional and defaults to "../RLBot".

Should it output any errors in white text, modify RLBot's source to fix
the errors. When the command completes successfully, you will have 
errors in red text. As long as the errors are in red, that means it
worked!
