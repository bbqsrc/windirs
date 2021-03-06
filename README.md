# windirs

[![Documentation](https://docs.rs/windirs/badge.svg)](https://docs.rs/windirs)

A safe wrapper around `SHGetKnownFolderPath`.

## Usage

```rust
use windirs::{Error, FolderId, get_known_folder};

let local_app_data_path = match get_known_folder(FolderId::LocalAppData) {
    Ok(path) => path,
    Err(err) => match err {
        // Some folder ids point to virtual paths, so, yeah.
        Error::Virtual => panic!(),
        // When no folder is found.
        Error::NotFound => panic!(),
        // Can occur for a variety of reasons, such as this folder id being unknown to this OS.
        Error::InvalidArg(io_error) => panic!(),
        // Any other potential OS error that could happen, but is not defined by the API.
        Error::Other(io_error) => panic!(),
    }
}

let user_home_path = get_known_folder(FolderId::Profile).unwrap();
```

### License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
