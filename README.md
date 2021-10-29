# THIS DOES NOT WORK YET

## Taggit
Taggit is a simple file tagging system that works slightly similar to Git.

### Why Not SQL?
The goal of Taggit was to make a tool that integrated well with the basic CLI.
For example, you can just `ls` the folder of a tag to see which files have that tag!
```
> ls suspense
explosions.pdf
boo.pdf
```

Or, search for an entry that matches the tags suspense, horror, and spooky in a nice, clean format.
`find ./(suspense|horror|spooky) -type l | xargs basename -a | uniq -d`

If these commands seem intimidating, don't worry. There is basic search functionality baked in.
`taggit list --name boo --tags spooky suspense horror`
or for short:
`taggit list -n boo -t spooky suspense horror`

### Git Compatibility
Taggit should work in a Git repository.

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
