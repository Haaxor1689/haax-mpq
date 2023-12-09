# haax-mpq

CLI utility to build MPQ archive from a folder.

## Usage

```
haax-mpq <directoryPath> <archivePath?>
```

- `directoryPath`: Full path to the folder
- `archivePath`: Full path to the output MPQ file. Default: `<directoryPath>.mpq`

## Patchignore

You can add a `.patchignore` file to your archive folder that will list rules about what files shouldn't be included in the built archive. It uses a similar syntax to `.gitignore` files (using [anymatch](https://github.com/micromatch/anymatch) library).

If no `.patchignore` file is found, these defaults will be used:

```
# ignore git related stuff
.git/**
.github/**
.gitignore

# ignore all file formats that have nothing to do in patch
**/*.json
**/*.yml
**/*.yaml
**/*.exe
**/*.dll
**/*.db
**/*.csv
**/*.png
**/*.psd
**/*.txt
**/*.md
**/*.sql

# ignore special mpq files
(listfile)
(attributes)
(signature)
(user data)

.patchignore
```

## Executable

You can also use this CLI as a standalone executable that can be found in Releases. Download `haax-mpq.exe` for Windows or `haax-mpq` for Linux. You can then drag & drop any folder you want to archive onto the executable.
