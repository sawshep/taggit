use clap::ArgMatches;
use std::io::Error;

/**
 * Query the list of checksums tracked by Taggit, optionally filtering by name and tags.
 */
fn list(matches: &ArgMatches) -> Result<(), Error> {
    // Never panics
    /*
     * These are just commented out right now so I don't get the
     * annoying warnings when building.
     */
    //let input_dir: &str = matches.value_of("archive").unwrap();

    //let archive = taggit::Archive::from_path(Path::new(input_dir))?;
    //for entry in archive.entries.entries {}

    if matches.is_present("name") {
        // Never panics
        let name: &str = matches.value_of("name").unwrap();
        println!("Sorting by {}", name);
    }
    if matches.is_present("tags") {
        // never panics
        let tags: Vec<&str> = matches.values_of("tags").unwrap().collect();
        println!("Sorting by {:?}", tags);
    }
    Ok(())
}
