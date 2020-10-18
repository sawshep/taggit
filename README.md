## Taggit
Taggit is a simple file tagging system that works slightly similar to Git.

### Git Compatibility
Taggit should work in a Git repository.
If a Taggit archive is initialized in a folder that is a Git repository, `/.taggit` is appended to `/.gitignore`. This prevents a cycle of commiting changes to both the Taggit and Git repositories.

### Scope
The scope of Taggit shall not expand out of these basic functions:
* Tagging of files based on checksums.
* Consistent tracking of tags over files with the same checksum.
* Management of tags associated with those checksums.
* Search for files in the archive based on name and tags.

Taggit will never implement these functions:
* Remote archives.
* Tracking file changes.

### Usage
See `taggit --help`.
