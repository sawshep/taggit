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
use std::fs;
use std::io::{prelude::*, BufReader, Error, Read};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Serialize, Deserialize)]
struct Entry {
    hash: String,
    names: Vec<String>,
    tags: Vec<String>,
}

impl Entry {
    /**
     * Moves the attributes of the provided
     * entry, excluding the SHA1 hash, into
     * the vectors of `self`. Also deletes
     * duplicates.
     */
    fn combine(&mut self, entry: &mut Entry) {
        self.names.append(&mut entry.names);
        self.names.sort();
        self.names.dedup();

        self.tags.append(&mut entry.tags);
        self.tags.sort();
        self.tags.dedup();
    }
}

/**
 * A bit of a wrapper class to
 * make the code below more linear
 */
#[derive(Serialize, Deserialize)]
struct Entries {
    entries: Vec<Entry>,
}

impl Entries {
    /**
     * If there is a matching entry in the Entries,
     * returns its index and a reference to the entry.
     */
    fn get_match(&mut self, compare: &Entry) -> Option<&mut Entry> {
        for entry in self.entries.iter_mut() {
            if entry.hash == compare.hash {
                return Some(entry);
            }
        }
        None
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn push(&mut self, value: Entry) {
        self.entries.push(value);
    }
}

// TODO: impl Iterator for Entries

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
        eprintln!("Error: an unknown error occurred");
    }
    /* --------------------------------- */

    exit_code
}

/**
 * Initialize a new Taggit archive, either in an existing folder or a new one.
 */
fn init(matches: &ArgMatches) -> i32 {
    // Never panics
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
    // Checks if the archive exists
    let taggit_path = Path::new(input_dir).join(TAGGIT_FOLDER);
    if !taggit_path.is_dir() {
        eprintln!("Taggit archive does not exist in {}", input_dir);
        return 1;
    }

    let input_files: Vec<&str> = matches
        .values_of("files")
        .unwrap() // Never panics
        .collect();

    let structs_path = taggit_path.join("structs.json");
    let mut structs_file = match fs::OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
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

    /* I've chosen to attempt to load
     * the file into memory as late as
     * possible. In theory, this should
     * make the program run and terminate
     * faster if the user makes an error
     * before this point. I'll have to
     * check this logic, though.
     */
    let mut entries = Entries {
        entries: Vec::new(),
    };
    let reader = BufReader::new(&structs_file);
    for line in reader.lines() {
        // Handle this panic
        let entry: Entry = serde_json::from_str(line.unwrap().as_str()).unwrap();
        entries.push(entry);
    }

    for input_file in input_files {
        let file_path = Path::new(input_file);
        if !file_path.is_file() {
            eprintln!("Cannot add {} to archive: is a directory", input_file);
            continue;
        }

        // Calculate the SHA1 sum
        let hash = match sha1sum(&file_path) {
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

        let mut new = Entry {
            hash: hash,
            names: vec![name],
            tags: tags,
        };

        match entries.get_match(&new) {
            Some(old) => {
                old.combine(&mut new);
            }
            None => {
                entries.push(new);
            }
        }
        for entry in &entries.entries {
            // Handle this panic
            let json = serde_json::to_string(&entry).unwrap() + "\n";
            match structs_file.write_all(json.as_bytes()) {
                Ok(r) => r,
                Err(_) => {
                    eprintln!("Failed to write to structs file");
                    return 1;
                }
            }
        }
    }
    0
}

/**
 * Find the sha1 checksum of a file.
 */
fn sha1sum(path: &Path) -> Result<String, Error> {
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
    // Never panics
    let input_dir: &str = matches.value_of("archive").unwrap();
    let taggit_path = Path::new(input_dir).join(TAGGIT_FOLDER);
    if !taggit_path.is_dir() {
        eprintln!("Taggit archive does not exist in {}", input_dir);
        return 1;
    }
    // Never panics
    let checksums: Vec<&str> = matches.values_of("checksums").unwrap().collect();
    // ^ TODO: Make user interface with tag work based on file IDs, not checksums.
    // (Still make tags associate with checksums, though)
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
    0
}

/**
 * Query the list of checksums tracked by Taggit, optionally filtering by name and tags.
 */
fn list(matches: &ArgMatches) -> i32 {
    // Never panics
    let input_dir: &str = matches.value_of("archive").unwrap();
    let taggit_path = Path::new(input_dir).join(TAGGIT_FOLDER);
    if !taggit_path.is_dir() {
        eprintln!("Taggit archive does not exist in {}", input_dir);
        return 1;
    }
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
    0
}
