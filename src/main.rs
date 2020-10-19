/**
 * A command-line file tagging/archiving utility.
 */
/* Some `unwrap()` statements do not need
 * to be handled due to clap. In all cases,
 * an argument is either required or has
 * a default value.
 */
use clap::{load_yaml, App, ArgMatches};
use serde::{Deserialize, Serialize};
use serde_json;
use sha1::Sha1;
use std::ffi::OsStr;
use std::fs;
use std::io::{Error, Read};
use std::path::{Path, PathBuf};
use std::process::exit;

//#[derive(Serialize, Deserialize, Debug)]
struct Entry<'a> {
    hash: String,
    names: &'a Vec<&'a OsStr>,
    tags: &'a Vec<&'a str>,
}

const TAGGIT_FOLDER: &'static str = ".taggit";

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
fn init(matches: &ArgMatches) -> i32 {
    let input_dir: &str = matches.value_of("directory").unwrap();
    let target_path = Path::new(&input_dir);
    let taggit_path: PathBuf = target_path.join(TAGGIT_FOLDER);
    if taggit_path.exists() {
        eprintln!("Error: Taggit archive already exists in {}", &input_dir);
        return 1;
    }
    match fs::create_dir_all(taggit_path.join("files")) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Error: could not create Taggit archive in {}", input_dir);
            return 1;
        }
    }

    println!("Initialized Taggit archive in {}", input_dir);
    0
}

/**
 * Add files to be tracked by Taggit, adding optional tags.
 */
fn add(matches: &ArgMatches) -> i32 {
    // Never panics
    let input_dir: &str = matches.value_of("archive").unwrap();
    let taggit_path = Path::new(input_dir).join(TAGGIT_FOLDER);
    if !taggit_path.is_dir() {
        eprintln!("Taggit archive does not exist in {}", input_dir);
        return 1;
    }
    // Never panics
    let input_files: Vec<&str> = matches.values_of("files").unwrap().collect();
    let structs_path = taggit_path.join("structs");
    let mut structs = match fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&structs_path)
    {
        Ok(s) => s,
        Err(_) => {
            eprintln!(
                "Error: could not open structures file at {}",
                structs_path.to_string_lossy()
            );
            return 1;
        }
    };
    for input_file in input_files {
        let file_path = Path::new(input_file);
        if !file_path.is_file() {
            eprintln!("Cannot add {} to archive: is a directory", input_file);
            continue;
        }
        let hash = match checksum(&file_path) {
            Ok(h) => h,
            Err(_) => {
                eprintln!("Cannot open {}, skipping", input_file);
                continue;
            }
        };
        let name = &file_path.file_name().unwrap();
        let mut entry = Entry {
            hash: hash,
            names: &vec![&name],
            tags: &vec![],
        };
        if matches.is_present("tags") {
            let input_tags: Vec<&str> = matches.values_of("tags").unwrap().collect();
            println!("{:?}", &input_tags);
            entry.tags = &input_tags;
        }
    }
    0
}

/**
 * Find the checksum of a file.
 */
fn checksum(path: &Path) -> Result<String, Error> {
    let mut file = match fs::File::open(path) {
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

//fn serialize(path: PathBuf) {}

/**
 * Manage tags associated with checksums.
 */
fn tag(matches: &ArgMatches) -> i32 {
    let input_dir: &str = matches.value_of("archive").unwrap();
    let taggit_path = Path::new(input_dir).join(TAGGIT_FOLDER);
    if !taggit_path.is_dir() {
        eprintln!("Taggit archive does not exist in {}", input_dir);
        return 1;
    }
    let checksums: Vec<&str> = matches.values_of("checksums").unwrap().collect();
    // ^ TODO: Make user interface with tag work based on file IDs, not checksums.
    // (Still make tags associate with checksums, though)
    if matches.is_present("add") {
        let tags: Vec<&str> = matches.values_of("add").unwrap().collect();
        println!("Associated {:?} with {:?}", tags, checksums);
    }
    if matches.is_present("delete") {
        let tags: Vec<&str> = matches.values_of("delete").unwrap().collect();
        println!("Dissociated {:?} with {:?}", tags, checksums);
    }
    0
}

/**
 * Query the list of checksums tracked by Taggit, optionally filtering by name and tags.
 */
fn list(matches: &ArgMatches) -> i32 {
    let input_dir: &str = matches.value_of("archive").unwrap();
    let taggit_path = Path::new(input_dir).join(TAGGIT_FOLDER);
    if !taggit_path.is_dir() {
        eprintln!("Taggit archive does not exist in {}", input_dir);
        return 1;
    }
    if matches.is_present("name") {
        let name: &str = matches.value_of("name").unwrap();
        println!("Sorting by {}", name);
    }
    if matches.is_present("tags") {
        let tags: Vec<&str> = matches.values_of("tags").unwrap().collect();
        println!("Sorting by {:?}", tags);
    }
    0
}
