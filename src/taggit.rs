/**
 * A module for the key
 * structures of taggit.
 */
use io::{BufRead, BufReader, Write};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io;
use std::path::{Path, PathBuf};

pub const FILES_FOLDER: &'static str = ".taggit/files";
pub const HASHES_FILE: &'static str = ".taggit/hashes.json";

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

/**
 * Holds information about the archive,
 * mostly paths to everything.
 */
#[derive(Debug)]
pub struct Archive {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
}

impl Archive {
    /**
     * Deserializes the entries in a
     * pre-existing archive.
     */
    pub fn from_path(path: &Path) -> Result<Self, io::Error> {
        let hashes: PathBuf = path.join(HASHES_FILE);
        let mut entries: Vec<Entry> = vec![];
        {
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(hashes)?;
            let reader = BufReader::new(file);

            for item in reader.lines() {
                let line: String = item?;
                let json: &str = line.as_str();
                let entry: Entry = serde_json::from_str(json)?;
                entries.push(entry);
            }
            println!("6");
        }

        let archive = Archive {
            path: path.to_path_buf(),
            entries: entries,
        };
        Ok(archive)
    }

    /* As of now, this loads every struct in the file into memory along
     * with any new structs, wipes over the file, then writes every
     * struct in memory to the file. This could get really memory
     * inefficient in very large archives. */
    pub fn write(&self) -> Result<(), io::Error> {
        /* I want the entries to be separated by newlines, so I open the
         * file with the truncate option then write over it with an
         * empty string first. Then, I open the same file with the
         * append option and write every struct on a different line. */
        {
            let mut truncator = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.path.join(HASHES_FILE))?;
            truncator.write_all(b"")?;
        }
        {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .append(true)
                .open(&self.path.join(HASHES_FILE))?;
            for entry in &self.entries {
                let json = serde_json::to_string(&entry)? + "\n";
                file.write_all(json.as_bytes())?;
            }
        }
        Ok(())
    }

    /**
     * If there is a matching Entry in the entries,
     * gets a mutable reference to the Entry.
     */
    pub fn get_match_mut(&mut self, compare: &Entry) -> Option<&mut Entry> {
        for entry in self.entries.iter_mut() {
            if entry.hash == compare.hash {
                return Some(entry);
            }
        }
        None
    }
}

//**
// * A bit of a wrapper class to
// * make the code below more linear
// */
//#[derive(Debug)]
//pub struct Entries {
//    // TODO: Make this private and impl iterator
//    pub entries: Vec<Entry>,
//    pub path: PathBuf,
//}
//impl Entries {
//    pub fn new() -> Self {
//        Entries {
//            entries: Vec::new(),
//            path: PathBuf::new(),
//        }
//    }
//
//    pub fn from_path(path: &Path) -> Result<Self, io::Error> {
//        use io::{BufRead, BufReader};
//
//        let file = OpenOptions::new().read(true).open(path)?;
//        let mut entries = Entries::new();
//        let reader = BufReader::new(file);
//
//        /*
//         * Find a better way to do this?
//         * Preferably, I would like to use the `try!` macro (or the `?`
//         * operator) in the for loop hearder.
//         */
//        for item in reader.lines() {
//            let line: String = item?;
//            let json: &str = line.as_str();
//            let entry: Entry = serde_json::from_str(json)?;
//            entries.push(entry);
//        }
//        Ok(entries)
//    }
