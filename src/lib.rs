//! LZMA/XZ encoding and decoding streams
//!
//! This library is a binding to liblzma currently to provide LZMA and xz
//! encoding/decoding streams. I/O streams are provided in the `read`, `write`,
//! and `bufread` modules (same types, different bounds). Raw in-memory
//! compression/decompression is provided via the `stream` module and contains
//! many of the raw APIs in liblzma.
//!
//! # Examples
//!
//! ```
//! use std::io::prelude::*;
//! use xz2::read::{XzEncoder, XzDecoder};
//!
//! // Round trip some bytes from a byte source, into a compressor, into a
//! // decompressor, and finally into a vector.
//! let data = "Hello, World!".as_bytes();
//! let compressor = XzEncoder::new(data, 9);
//! let mut decompressor = XzDecoder::new(compressor);
//!
//! let mut contents = String::new();
//! decompressor.read_to_string(&mut contents).unwrap();
//! assert_eq!(contents, "Hello, World!");
//! ```
//! # Static linking
//!
//! You can enable static-linking using the `static` feature, so that the XZ
//! library is not required at runtime:
//!
//! ```toml
//! xz2 = { version = "0.1.6", features = ["static"] }
//! ```

#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/xz2/0.1")]

pub mod stream;

pub mod bufread;
pub mod read;
pub mod write;
