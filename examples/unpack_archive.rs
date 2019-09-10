use std::path::PathBuf;
use tar::Archive;
use xz2::read::XzDecoder;

fn unpack_tar_xz(archive_path: &PathBuf, dest: &PathBuf) {
    let archive_bytes = fs::read(archive_path).expect("Problem reading archive as bytes");

    let mut tar: Vec<u8> = Vec::new();
    let mut decompressor = XzDecoder::new(&archive_bytes[..]);
    decompressor.read_to_end(&mut tar).expect("Problem decompressing archive");

    // We've decompressed the .xz; now unpack the tar.
    let mut archive = Archive::new(&tar[..]);
    archive.unpack(dest).expect("Problem unpacking tar");
}