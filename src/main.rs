/**
 * A command-line file tagging/archiving utility.
 */
/* Some `unwrap()` statements do not need
 * to be handled due to clap. In all cases,
 * an argument is either required or has
 * a default value.
 */
use clap::{load_yaml, App, ArgMatches};
use std::fs;
use std::path::{Path, PathBuf};
//use std::process::exit;
use std::io::{Error, ErrorKind};

mod common;
mod taggit;

fn main() -> Result<(), Error> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let exit_code: i32;

    if let Some(ref matches) = matches.subcommand_matches("init") {
        init(matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("add") {
        add(matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("tag") {
        tag(matches)?;
    } else if let Some(ref matches) = matches.subcommand_matches("list") {
        list(matches)?;
    }
    Ok(())
}

/**
 * Initialize a new Taggit archive,
 * either in an existing folder or a new one.
 */
fn init(matches: &ArgMatches) -> Result<(), Error> {
    // Never panics
    let input_dir: &str = matches.value_of("directory").unwrap();
    let target_path = Path::new(&input_dir);
    let taggit_path: PathBuf = target_path.join(taggit::TAGGIT_FOLDER);
    let msg = format!("Archive already exists in {}", input_dir);
    if taggit_path.exists() {
        return Err(Error::new(ErrorKind::AlreadyExists, msg));
    }
    fs::create_dir_all(taggit_path.join("files"))?;
    println!("Initialized Taggit archive in {}", input_dir);
    Ok(())
}

/**
 * Add files to be tracked by Taggit, adding optional tags.
 */
fn add(matches: &ArgMatches) -> Result<(), Error> {
    // Never panics
    let input_dir: &str = matches.value_of("archive").unwrap();

    let archive = taggit::Archive::from_path(Path::new(input_dir))?;

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

        match archive.entries.get_match_mut(&new) {
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

//fn serialize(path: PathBuf) {}

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

    for entry in archive.entries.entries {
        for hash in checksums {
            if hash == entry.hash {}
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

/**
 * Query the list of checksums tracked by Taggit, optionally filtering by name and tags.
 */
fn list(matches: &ArgMatches) -> Result<(), Error> {
    // Never panics
    let input_dir: &str = matches.value_of("archive").unwrap();

    let archive = taggit::Archive::from_path(Path::new(input_dir))?;

    for entry in archive.entries.entries {}

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
