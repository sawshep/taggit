use clap::ArgMatches;
use std::path::Path;
use std::io::Error;

mod taggit;

/**
 * Manage tags associated with checksums.
 */
fn tag(matches: &ArgMatches) -> Result<(), Error> {
    let input_dir: &str = matches.value_of("archive").unwrap();

    let archive = taggit::Archive::from_path(Path::new(input_dir))?;

    // Never panics
    let checksums: Vec<&str> = matches.values_of("checksums").unwrap().collect();
    // ^ TODO: Make user interface with tag work based on file IDs, not checksums.
    // (Still make tags associate with checksums, though)

    for entry in archive.entries {
        for hash in &checksums {
            if *hash == entry.hash {}
        }
    }
    if matches.is_present("add") {
        // Never panics
        let tags: Vec<&str> = matches.values_of("add").unwrap().collect();
        println!("Associated {:?} with {:?}", tags, checksums);
    }
    if matches.is_present("delete") {
        // Never panics
        let tags: Vec<&str> = matches.values_of("delete").unwrap().collect();
        println!("Dissociated {:?} with {:?}", tags, checksums);
    }
    Ok(())
}
