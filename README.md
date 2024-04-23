This is for an assignment to make a "virus" that searches a targets computer for an encrypted file and a key, decrypts the file, and uploads it to a server.

## Specific Behavior

* Targets x86_64 Ubuntu. It should work on all major PCs though.
* Searches for an encrypted file named `special_file.txt` or `secret_file.txt`. Despite thier suffixes, they are raw binary files. The locations searched is dependent on the feature flag.
* Decrypts ths file using AES-196. See [this repo](https://github.com/NCGThompson/Encryption-Tools-for-Project-3) for the exact protocol used.
* Posts the decrypted file to a local test server at `http://127.0.0.1:80`. If you want to try this out, you will have to set one up on your own. You may need to edit the source code if your server is listining on a different port.
* Iff all goes well, it should print nothing to stdout or stderr.

## Security

This is, in fact malware, so consider running in a sandboxed environment. However, it should be harmless to except for excessive disk usage and the behavior described above.

## `find_files`

This "example" executable is a cli tool intended to demonstrate the searching function of the executable in a more observable manner.
Coincadentally it is pretty useful on its own. It uses the very fast [`rust_search`](https://github.com/ParthJadhav/Rust_Search) library, but processes the input to allow multiple files being searched for at once while preventing accidental regex injection. By default, it searches almost everywhere, but prioritzes more likely directories depending on the OS, decreasing the average search time. Start with `cargo run --example find_files --release -- --help`.

## Project Structure

While this project is dedicated to a single executable, most of the code is part of a Rust library, `libproj3` to allow doctests and examples. The side effect of this is that most internal documentation is not displayed by default. This can be over-ridden with `--document-private-items`. Other than that this is a standard Cargo project. See [Cargo's documentation](https://doc.rust-lang.org/cargo/) for usage tips.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
