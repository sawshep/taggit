use clap::ArgMatches;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

mod common;
mod taggit;

/**
 * Add files to be tracked by Taggit, adding optional tags.
 */
fn add(matches: &ArgMatches) -> Result<(), Error> {
    // Never panics
    let input_dir: &str = matches.value_of("archive").unwrap();

    let mut archive = taggit::Archive::from_path(Path::new(input_dir))?;

    let input_files: Vec<&str> = matches
        .values_of("files")
        .unwrap() // Never panics
        .collect();

    for input_file in input_files {
        let file_path = Path::new(input_file);
        if !file_path.is_file() {
            eprintln!("Cannot add {} to archive: is a directory", input_file);
            continue;
        }

        // Calculate the SHA1 sum
        let hash = match common::sha1sum(&file_path) {
            Ok(h) => h,
            Err(_) => {
                eprintln!("Cannot open {}, skipping", input_file);
                continue;
            }
        };

        // Find the name of the file.
        // This included the extension.
        // Convert it into a String so
        // it looks pretty in JSON.
        let name: String = file_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();

        // Make the tag arguments into Strings
        // so they look pretty in JSON.
        let mut tags = vec![];
        if matches.is_present("tags") {
            // Never panics
            let input_tags: Vec<&str> = matches.values_of("tags").unwrap().collect();
            for tag in &input_tags {
                tags.push(String::from(*tag));
            }
        }

        let mut new = taggit::Entry {
            hash: hash,
            names: vec![name],
            tags: tags,
        };

        match archive.get_match_mut(&new) {
            Some(old) => {
                old.combine(&mut new);
            }
            None => {
                archive.entries.push(new);
            }
        }
    }
    archive.write()?;
    Ok(())
}
