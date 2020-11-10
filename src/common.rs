/**
 * Common functions used throughout
 * the program that are not necessarially
 * related to taggit.
 */
use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

/**
 * Find the sha1 checksum of a file
 * from a path.
 */
pub fn sha1sum(path: &Path) -> Result<String, Error> {
    use sha1::Sha1;
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut hasher = Sha1::new();
    let mut bytes = Vec::new();
    match file.read_to_end(&mut bytes) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };
    hasher.update(&bytes);
    Ok(hasher.digest().to_string())
}
