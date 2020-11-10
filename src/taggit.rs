/**
 * A module for the key
 * structures of taggit.
 */
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io;
use std::path::{Path, PathBuf};

pub const TAGGIT_FOLDER: &'static str = ".taggit";
pub const FILES_FOLDER: &'static str = "files";
pub const ENTRIES_FILE: &'static str = "entries.json";

/**
 * Holds information about the archive,
 * mostly paths to everything.
 */
#[derive(Debug)]
pub struct Archive {
    pub path: PathBuf,
    // TODO: Change to private, impl iterator in Entries
    pub taggit_path: PathBuf,
    pub entries_path: PathBuf,
    pub files_path: PathBuf,

    pub entries: Entries,
}

impl Archive {
    // TODO: Make this return a result
    /**
     * Deserializes the entries in a
     * pre-existing archive.
     */
    pub fn from_path(archive_path: &Path) -> Result<Self, io::Error> {
        let taggit_path = archive_path.join(TAGGIT_FOLDER);
        let files_path = taggit_path.join(FILES_FOLDER);
        let entries_path = taggit_path.join(ENTRIES_FILE);

        let entries = Entries::from_path(&entries_path)?;

        let archive = Archive {
            path: archive_path.to_path_buf(),
            taggit_path: taggit_path,
            // TODO: Handle this panic
            entries: entries,
            entries_path: entries_path,
            files_path: files_path,
        };
        Ok(archive)
    }

    pub fn write(&self) -> Result<(), io::Error> {
        let mut truncator = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(self.entries_path)?;
        truncator.write_all(b"")?;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(self.entries_path)?;
        for entry in self.entries.entries {
            let json = serde_json::to_string(&entry)? + "\n";
        }
        Ok(())
    }

    // TODO: Provide a method for creating the files of an archive
    /*
    // TODO: Rename this to create or something? I think `new` is sacred.
    pub fn new(path: &Path) -> Result<Self, std::io::Error> {
        Ok(archive)
    }
    */
}

// TODO: impl Iterator for Entries
/**
 * A bit of a wrapper class to
 * make the code below more linear
 */
#[derive(Debug)]
pub struct Entries {
    // TODO: Make this private and impl iterator
    pub entries: Vec<Entry>,
}
impl Entries {
    pub fn new() -> Self {
        Entries {
            entries: Vec::new(),
        }
    }

    pub fn from_path(path: &Path) -> Result<Self, io::Error> {
        use io::{BufRead, BufReader};

        let file = OpenOptions::new().read(true).open(path)?;
        let mut entries = Entries::new();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let json: &str = line?.as_str();
            let entry: Entry = serde_json::from_str(json)?;
            entries.push(entry);
        }
        Ok(entries)
    }
    /**
     * If there is a matching entry in the Entries,
     * gets a mutable reference to the entry.
     */
    pub fn get_match_mut(&mut self, compare: &Entry) -> Option<&mut Entry> {
        for entry in self.entries.iter_mut() {
            if entry.hash == compare.hash {
                return Some(entry);
            }
        }
        None
    }

    // TODO: There may be a std implementation or a derive for push and contains
    pub fn push(&mut self, value: Entry) {
        self.entries.push(value);
    }

    pub fn contains(&self, value: &Entry) -> bool {
        self.entries.contains(value)
    }
}
//impl Iterator for Entries {}

/**
 * An entry into taggit, containing
 * a file hash, all the filenames
 * the hash has ever appeared under,
 * and the user-added tags.
 */
// For use with Serde
#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub hash: String,
    pub names: Vec<String>,
    pub tags: Vec<String>,
}
impl Entry {
    /**
     * Moves the attributes of the provided
     * entry, excluding the SHA1 hash, into
     * the vectors of `self`. Also deletes
     * duplicates.
     */
    pub fn combine(&mut self, other: &mut Self) {
        self.names.append(&mut other.names);
        self.names.sort();
        self.names.dedup();

        self.tags.append(&mut other.tags);
        self.tags.sort();
        self.tags.dedup();
    }
}
impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
