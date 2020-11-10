use serde::{Deserialize, Serialize};

/**
 * An entry into taggit, containing
 * a file hash, all the filenames
 * the hash has ever appeared under,
 * and the user-added tags.
 */
// For use with Serde
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
struct Entries {
    entries: Vec<Entry>,
}

// TODO: impl Iterator for Entries

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

    fn push(&mut self, value: Entry) {
        self.entries.push(value);
    }
}
