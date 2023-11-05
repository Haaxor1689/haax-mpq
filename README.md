# haax-mpq

CLI utility to build MPQ archive from a folder.

## Usage

```
haax-mpq <directoryPath> <archivePath?>
```

- `directoryPath`: Full path to the folder
- `archivePath`: Full path to the output MPQ file. Default: `<directoryPath>.mpq`

## Executable

You can also use this CLI as a standalone executable. Run these scripts after cloning to generate it:

```
pnpm i
pnpm run bundle
```

This will create a `haax-mpq` executable in the root of this repository. You can then drag & drop any folder you want to archive onto the executable.
