# 🦀🤖 Resizinator
Simple tool for debugging issues with window resizing by spamming resize commands to a window.

### Dependencies
* `wmctrl`  (install via ```sudo apt install wmctrl```)
* Rust compiler (install from https://rustup.rs/)

### How to run:
* Find your window name using ```wmctrl -l```
* Modify `main.rs` with your window name, screen resolution and DPI scale
* Start resizinator ```cargo run```

You can stop resizinator by using `Ctrl + C`.

### License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](../master/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../master/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
