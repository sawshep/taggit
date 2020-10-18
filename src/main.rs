use clap::{load_yaml, App};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use std::fs::{create_dir_all, set_permissions, File};
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Serialize, Deserialize)]
struct Entry {
    id: u32,
    name: String,
    hash: String,
    tags: Vec<String>,
}

/**
 * The exit function has been separated from `taggit()`
 * (which would have been `main()`) as it does not run
 * any destructors.
 */
fn main() {
    exit(taggit());
}

fn taggit() -> i32 {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let exit_code: i32;

    /* Make this into a match statement? */
    if let Some(ref matches) = matches.subcommand_matches("init") {
        exit_code = init(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("add") {
        exit_code = add(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("tag") {
        exit_code = tag(matches);
    } else if let Some(ref matches) = matches.subcommand_matches("list") {
        exit_code = list(matches);
    } else {
        exit_code = 1;
        eprintln!("Error: an unknown error occured");
    }
    /* --------------------------------- */

    exit_code
}

/**
 * Initialize a new Taggit archive, either in an existing folder or a new one.
 */
fn init(matches: &clap::ArgMatches) -> i32 {
    if matches.is_present("directory") {
        let directory: &str = matches.value_of("directory").unwrap();
        let target = Path::new(directory);
        let taggit: PathBuf = target.join(".taggit");
        if taggit.exists() {
            eprintln!("Error: Taggit archive already exists in {}", directory);
            return 1;
        }
        match create_dir_all(&taggit) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Error: could not create Taggit archive in {}", directory);
                return 1;
            }
        }
        println!("Initialized Taggit archive in {}", taggit.to_string_lossy());
    }
    0
}

/**
 * Add files to be tracked by Taggit, adding optional tags.
 */
fn add(matches: &clap::ArgMatches) -> i32 {
    let files: Vec<_> = matches.values_of("files").unwrap().collect();
    for file in files {
        let path = PathBuf::from(file);
        if path.is_file() {
            match checksum(path) {
                Ok(hash) => println!("{}", hash),
                Err(_) => {
                    eprintln!("Error: could not calculate hash of {}", file);
                    continue;
                }
            };
        } else {
            eprintln!("Cannot add {} to archive: is a directory", file);
        }
    }
    if matches.is_present("tags") {
        let tags: Vec<_> = matches.values_of("tags").unwrap().collect();
        println!("{:?}", tags);
    }
    0
}

/**
 * Find the checksum of a file.
 */
fn checksum(path: PathBuf) -> Result<String, Error> {
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

/**
 * Manage tags associated with checksums.
 */
fn tag(matches: &clap::ArgMatches) -> i32 {
    let checksums: Vec<_> = matches.values_of("checksums").unwrap().collect();
    if matches.is_present("add") {
        let tags: Vec<_> = matches.values_of("add").unwrap().collect();
        println!("Associated {:?} with {:?}", tags, checksums);
    }
    if matches.is_present("delete") {
        let tags: Vec<_> = matches.values_of("delete").unwrap().collect();
        println!("Dissociated {:?} with {:?}", tags, checksums);
    }
    0
}

/**
 * Query the list of checksums tracked by Taggit, optionally filtering by name and tags.
 */
fn list(matches: &clap::ArgMatches) -> i32 {
    if matches.is_present("name") {
        let name: &str = matches.value_of("name").unwrap();
        println!("Sorting by {:?}", name);
    }
    if matches.is_present("tags") {
        let _tags: Vec<_> = matches.values_of("tags").unwrap().collect();
    }
    0
}
