# haax-mpq

CLI utility to build MPQ archive from a folder.

## Usage

```
haax-mpq <input> <output>
```

- Providing MPQ path as `input` extracts it into given `output` folder
- Providing folder path as `input` packages it into a MPQ archive at give `output` path

## Patchignore

You can add a `.patchignore` file to your archive folder that will list rules about what files shouldn't be included in the built archive. It uses a similar syntax to `.gitignore` files.

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
