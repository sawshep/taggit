GOAL: The goal of this project was to create a simple file tagging system. I wanted to not use metadata stored in the actual files, and make the files searchable with the core command line utilities. I also wanted text-based configuration for advanced users.

RESEARCH & IMAGINING: The seed of my idea came from the version-control system called Git. Git is very complicated and does many things, but the basic idea is to track what files changed and which ones. I was only interesed in how it determined if a file changed.
Git uses the Secure Hashing Algorithm 1 to calculate a 20 digit hexidecimal number that is unique to the data inputted (Technically, there can be collisions, but those are a non-issue for this project).
My basic design idea was to associate file tags with the checksums of a file. This means that if a user added a file to the archive with the tag "horror", and then later added an identical file with the tag "scary", the tags would both be associated with the same checksum. I then would need to store this data in a file for when the program was run the next time.
I would have four basic commands: init, add, list, and manage. Below is a description of the function of each command.
init: Initializes the directory provided in the argument (or the current one, if none is provided) as a Taggit archive.
    Creates the hidden ".taggit" directory, which contains both a JSON file holding the metadata about each hash, and a folder in which the actual file data is moved to later by the add command.
add: This is the most complicated command. Adds a file to the taggit archive.
    Calculates the checksum of the files passed as arguments, and sees if any are identical to other entries in the archive. If they are, add the tags and filenames passed as flag arguments to the entry. If they are not, create new entries in the archive with this data, and move the actual file data to a hidden directory, named as the file's checksum. A directory is created for every tag that exists, with a symlink inside that links to the file within the hidden directory, if it has that tag.
list: Lists the entries in an archive. Also filters based on tag and name flags.
manage: 

PLAN:

CREATE:

EVALUATION & REDESIGN:
