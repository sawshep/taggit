/**
 * Initialize a new Taggit archive, either in an existing folder or a new one.
 */

use clap::ArgMatches;
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};

mod taggit;

/**
 * Initialize a new Taggit archive, either in an existing folder or a new one.
 */
fn init(matches: &ArgMatches) -> Result<(), Error> {
    // Never panics
    let input: &str = matches.value_of("directory").unwrap();
    let path = Path::new(&input);
    let taggit: PathBuf = path.join(".taggit");

    let msg = format!("Archive already exists in {}", input);
    if taggit.exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, msg));
    }
    fs::create_dir_all(path.join(taggit::FILES_FOLDER))?;
    println!("Initialized Taggit archive in {}", input);
    Ok(())
}
